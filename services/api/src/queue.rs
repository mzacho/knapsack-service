use uuid::Uuid;
use lapin::Connection;
use lapin::Channel;
use lapin::ConnectionProperties;

use backoff::ExponentialBackoff;
use backoff::future::retry;

use crate::get_var;

const AMQP_EXCHANGE: &str = "";
const AMQP_QUEUE_NAME: &str = "problem_submitted";

fn to_backoff_err(err: lapin::Error) -> backoff::Error<String> {
    use backoff::Error::Transient;

    // treat everything as transient errors with the default backoff time
    Transient { err: err.to_string(), retry_after: None }
}

pub async fn init() -> (Channel, Connection) {
    let uri = get_var("AMQP_ADDRESS");

    let conn = retry(ExponentialBackoff::default(), || async {
        println!("Connecting to AMQP broker");
        Connection::connect(&uri, ConnectionProperties::default()).await
            .map_err(to_backoff_err)}).await.unwrap();

    println!("Connected!");

    let chan = retry(ExponentialBackoff::default(), || async {
        println!("Creating AMQP channel");
        conn.create_channel().await.map_err(to_backoff_err)}).await.unwrap();

    println!("Created!");

    (chan, conn)
}

pub async fn publish_problem<'a>(task_id: &Uuid, chann: &Channel) {
    use lapin::BasicProperties;
    use lapin::options::BasicPublishOptions;

    let id_str = task_id.to_string();
    let payload = id_str.as_bytes();

    let _confirmation = retry(ExponentialBackoff::default(), || async {
        chann.basic_publish(
            AMQP_EXCHANGE,
            AMQP_QUEUE_NAME,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        ).await.map_err(to_backoff_err)}).await.unwrap();

    // todo: producer confirms currently not requested
}
