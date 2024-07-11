use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DbError;
pub use diesel::result::QueryResult;
use diesel::Connection as DieselConnection;
use rocket::http::Status;
use uuid::Uuid;

use std::ops::DerefMut;
use std::sync::Mutex;
use std::sync::MutexGuard;

use backoff::{retry, ExponentialBackoff, Error};

use crate::{get_var, ApiResult, HttpStatus, Knapsack};

pub mod models;
pub mod schema;

use models::Solution;
use models::Task;

pub type Connection = Mutex<PgConnection>;

pub async fn init() -> Connection {
    let user = get_var("DATABASE_USER");
    let pass = get_var("DATABASE_PASS");
    let host = get_var("DATABASE_HOST");

    let db = ""; // use default (postgres) database

    let url = format!("postgres://{}:{}@{}/{}", user, pass, host, db);

    let connection = retry(ExponentialBackoff::default(), || {
        PgConnection::establish(&url).map_err(Error::transient)}).unwrap();

    Mutex::new(connection)
}

pub fn insert(task: &Knapsack, conn: &Connection) -> ApiResult<()> {
    let task = Task::from_dto(task);

    let mut conn = grab_lock(conn)?;

    match retry(ExponentialBackoff::default(), || {
        diesel::insert_into(schema::tasks::table)
            .values(&task)
            .execute(conn.deref_mut()).map_err(Error::transient)})
    {
        Ok(rows_updated) => {
            if rows_updated != 1 {
                eprintln!("Warning: db: Expected 1, was {}", rows_updated)
            }
            Ok(())
        },
        Err(e) => match e {
            Error::Permanent(err) => Err(to_http_status(err)),
            Error::Transient { err, retry_after: _ } => Err(to_http_status(err)),
        }
    }
}

fn to_http_status(e: DbError) -> HttpStatus {
    use diesel::result::Error::*;
    eprintln!("{}", e);
    HttpStatus::new(match e {
        InvalidCString(_) => 400,
        DatabaseError(_, _) => 500,
        NotFound => 404,
        QueryBuilderError(_) => todo!(),
        DeserializationError(_) => todo!(),
        SerializationError(_) => todo!(),
        RollbackErrorOnCommit {
            rollback_error: _re,
            commit_error: _ce
        } => todo!(),
        RollbackTransaction => todo!(),
        AlreadyInTransaction => todo!(),
        NotInTransaction => todo!(),
        BrokenTransactionManager => todo!(),
        _ => todo!(),
    })
}

pub fn grab_lock(conn: &Connection) -> ApiResult<MutexGuard<PgConnection>> {
    match conn.lock() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            eprintln!("db: Could not grab lock: {}", e);
            Err(Status::new(500))
        }
    }
}

pub fn get(task_id: Uuid, conn: &Connection) -> ApiResult<Knapsack> {
    use schema::tasks;
    let mut conn = grab_lock(conn)?;

    let task = map_db_result(
        tasks::table
            .filter(tasks::id.eq(task_id))
            .select(Task::as_select())
            .get_result(conn.deref_mut())
    )?;

    let knapsack = Knapsack::from_task(&task)?;

    if matches!(task.status.as_str(), "completed") {
        let solution = map_db_result(
            Solution::belonging_to(&task)
                .select(Solution::as_select())
                .get_result(conn.deref_mut())
        )?;

        Ok(knapsack.set(solution)?)
    } else {
        Ok(knapsack)
    }
}

fn map_db_result<T>(res: QueryResult<T>) -> ApiResult<T> {
    match res {
        Ok(task) => Ok(task),
        Err(e) => Err(to_http_status(e))
    }
}
