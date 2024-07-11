use uuid::Uuid;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use diesel::prelude::*;
use diesel::Connection as DieselConnection;
use diesel::pg::PgConnection;

use backoff::{retry, ExponentialBackoff, Error};

pub mod schema;
pub mod models;

use models::Task;
use crate::get_var;

use self::models::Solution;

pub type Connection = Arc<Mutex<PgConnection>>;

pub async fn init() -> Connection {
    let user = get_var("DATABASE_USER");
    let pass = get_var("DATABASE_PASS");
    let host = get_var("DATABASE_HOST");

    // Connect to default (postgres) database
    let db = "";

    let url = format!("postgres://{}:{}@{}/{}", user, pass, host, db);

    let connection = retry(ExponentialBackoff::default(), || {
        PgConnection::establish(&url).map_err(Error::transient)})
        .expect("Could not connect to database");

    Arc::new(Mutex::new(connection))
}

pub fn set_task_status_started(task_id: Uuid, conn: &Connection) -> Result<Task, String> {

    use schema::tasks::dsl::*;

    let mut conn = grab_lock(conn)?;

    match diesel::update(schema::tasks::table)
        .filter(id.eq(task_id))
        .set((status.eq("started"),
              ts_started.eq(current_time()),
        ))
        .returning(Task::as_returning())
        .get_result(conn.deref_mut())
    {
        Ok(task) => Ok(task),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn insert_solution(solution: Solution, conn: &Connection) -> Result<(), String> {
    use schema::tasks::dsl::*;
    let mut conn = grab_lock(&conn)?;

    match diesel::insert_into(schema::solutions::table)
        .values(&solution)
        .execute(conn.deref_mut())
    {
        Ok(rows_updated) => {
            if rows_updated != 1 {
                eprintln!("Warning: db: Expected 1, was {}", rows_updated)
            }
            Ok(())
        },
        Err(e) => Err(e.to_string()),
    }?;

    match diesel::update(schema::tasks::table)
        .filter(id.eq(solution.task_id))
        .set((status.eq("completed"),
            ts_completed.eq(current_time())))
        .execute(conn.deref_mut())
    {
        Ok(rows_updated) => {
            if rows_updated != 1 {
                eprintln!("Warning: db: Expected 1, was {}", rows_updated)
            }
            Ok(())
        },
        Err(e) => Err(e.to_string()),
    }

    // todo: retry
}

// todo: use api as a lib to avoid code duplication

pub fn grab_lock(conn: &Connection) -> Result<MutexGuard<PgConnection>, String> {
    match conn.lock() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            eprintln!("db: Could not grab lock: {}", e);
            Err(format!("{}",e))
        }
    }
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
