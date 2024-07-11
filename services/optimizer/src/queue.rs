use lapin::options::QueueDeclareOptions;
use lapin::ConnectionProperties;
use lapin::types::FieldTable;
use lapin::options::BasicConsumeOptions;

use backoff::{ExponentialBackoff, Error};
use backoff::future::retry;

use crate::get_var;

const AMQP_EXCHANGE: &str = "";
const AMQP_QUEUE_NAME: &str = "problem_submitted";

type Connection = lapin::Connection;
type Channel = lapin::Channel;
type Consumer = lapin::Consumer;

pub async fn init() -> (Consumer, Channel, Connection) {
    let uri = get_var("AMQP_ADDRESS");

    let conn = retry(ExponentialBackoff::default(), || async {
        println!("Connecting to AMQP broker");
        Connection::connect(&uri, ConnectionProperties::default()).await
            .map_err(Error::transient)}).await.unwrap();

    println!("Connected!");

    let chan = retry(ExponentialBackoff::default(), || async {
        println!("Creating to AMQP channel");
        conn.create_channel().await.map_err(Error::transient)})
        .await.unwrap();

    println!("Created!");

    retry(ExponentialBackoff::default(), || async {chan.queue_declare(
        AMQP_QUEUE_NAME,
        QueueDeclareOptions::default(),
        FieldTable::default()
    ).await.map_err(Error::transient)}).await.unwrap();

    // todo: set channel prefetch maximum
    // chan.basic_qos(100, BasicQosOptions::default())

    let consumer = retry(ExponentialBackoff::default(), || async { chan.basic_consume(
        AMQP_EXCHANGE,
        AMQP_QUEUE_NAME,
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await.map_err(Error::transient)})
        .await.expect("Could not create AMQP consumer");

    println!("Consumer created");

    (consumer, chan, conn)
}
