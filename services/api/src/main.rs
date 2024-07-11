#[macro_use]
extern crate rocket;

mod dto;
mod queue;
mod db;

use uuid::Uuid;
use rocket::{Rocket, Orbit, State};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use rocket::http::Status;
use std::time::{SystemTime, UNIX_EPOCH};

use dto::{Knapsack, ProblemBody};

/// Gets the environment variable `name`, panicking if it's not set
fn get_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}

/// Returns the current unix/ epoch time
fn current_time() -> i32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .try_into()
        .expect("This program is way old and needs maintenance")
}

type HttpStatus = rocket::http::Status;
type ApiResult<T> = Result<T, HttpStatus>;

#[get("/knapsack/<id>")]
async fn status(id: &str, state: &State<Connections>) -> ApiResult<Json<Knapsack>> {
    if let Ok(id) = Uuid::try_parse(id) {
        db::get(id, &state.db_conn).map(Json)
    } else {
        // ill-formed id
        Err(Status::new(400))
    }
}

struct Connections {
    queue_conn: lapin::Connection,
    queue_chann: lapin::Channel,
    db_conn: db::Connection,
}

#[rocket::async_trait]
impl Fairing for Connections {
    async fn on_request(&self, _req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
    }
    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        println!("hep");
        self.queue_chann.close(0, "closing queue channel")
            .await.expect("closing queue channel failed");
        self.queue_conn.close(0, "closing queue connection")
            .await.expect("closing queue connection failed")
    }

    fn info(&self) -> Info {
        Info {
            name: "Shutdown connections",
            kind: Kind::Shutdown,
        }
    }
}

#[post("/knapsack", data = "<problem>")]
async fn submit(problem: Json<ProblemBody>, state: &State<Connections>) -> ApiResult<Json<Knapsack>> {
    problem.validate()?;
    let knapsack = Knapsack::new(problem.0.owned_to_problem());
    db::insert(&knapsack, &state.db_conn)?;
    queue::publish_problem(&knapsack.task, &state.queue_chann).await;
    Ok(Json(knapsack))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Connect to RabbitMQ and Postgres
    let (queue_chann, queue_conn) = queue::init().await;
    let db_client = db::init().await;

    // State to be managed by web framework
    let connections = Connections { queue_chann, queue_conn, db_conn: db_client };

    let _rocket = rocket::build()
        .manage(connections)
        .mount("/", routes![status, submit])
        .launch()
        .await?;

    Ok(())
}
