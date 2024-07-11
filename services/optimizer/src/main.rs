use futures_lite::stream::StreamExt;
use lapin::{message::Delivery, options::BasicAckOptions};
use uuid::Uuid;

use std::{sync::Arc, thread};

mod db;
mod queue;
mod solver;

struct Connections {
    queue_consumer: lapin::Consumer,
    queue_channel: lapin::Channel,
    queue_conn: lapin::Connection,
    db_conn: db::Connection,
}

#[tokio::main]
async fn main() -> ! {
    println!("Initializing...");
    let (queue_consumer, queue_channel, queue_conn) = queue::init().await;
    let db_conn = db::init().await;

    let mut connections = Connections {
        queue_consumer,
        queue_channel,
        queue_conn,
        db_conn,
    };

    loop {
        if let Some(delivery) = connections.queue_consumer.next().await {
            if let Ok(msg) = delivery {
                match consume(msg, &mut connections).await {
                    Ok(()) => println!("yay, consumed a message"),
                    Err(e) => eprintln!("{}", e),
                }
            } else {
                eprintln!("Warning: Error consuming message");
            }
        } else {
            eprintln!("Error in queue consumer")
        }
    }
}

use backoff::{retry, ExponentialBackoff, Error};

async fn consume(msg: Delivery, connections: &Connections) -> Result<(), String> {
    let data = &msg.data.clone();
    let data = map_err_to_string(std::str::from_utf8(data))?;
    let task_id = map_err_to_string(Uuid::parse_str(data))?;

    let task = db::set_task_status_started(task_id, &connections.db_conn)?;

    map_err_to_string(msg.ack(BasicAckOptions::default()).await)?;


    // Cloned the reference counted database mutex, so it can be moved
    // into the new thread
    let db_conn = Arc::clone(&connections.db_conn);

    // Spawn a new thread to compute the solution and update the db
    thread::spawn(move || {
        let solution = solver::solve(&task);
        if let Ok(solution) = solution {
            retry(ExponentialBackoff::default(), || {
                db::insert_solution(solution.clone(), &db_conn)
                    .map_err(Error::transient)
            }).expect("Failed to update db with solution")
        } else {
            eprintln!("Could not solve task {}", task.id)
        }
    });


    Ok(())
}

fn map_err_to_string<T, E>(res: Result<T, E>) -> Result<T, String>
where
    E: std::fmt::Display,
{
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    }
}

/// Gets the environment variable `name`, panicking if it's not set
fn get_var(name: &str) -> String {
    std::env::var(name).expect(&format!("{} not set", name))
}
