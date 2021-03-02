#![feature(toowned_clone_into)]

use std::{env};
use futures::*;

use rdkafka::{ClientContext, Message, TopicPartitionList};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tokio::sync::watch::{Sender, Receiver};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);
    let (tx, rx) = watch::channel("hello".to_string());

    let rt = tokio::runtime::Runtime::new().unwrap();



    let kafka_task =  async {
        // TODO: Change to environment variables
        let brokers = "localhost:9092";
        let topics = vec!["door-state"];
        let group_id = "consumer-1";

        consume(brokers, group_id, &topics, tx).await;
    };
    rt.spawn(kafka_task);


    while let Ok((stream, _)) = listener.accept().await {
        let  rx_clone = rx.clone();
        tokio::spawn(  {
            accept_connection(stream,  rx_clone)
        });

    }


    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}



async fn accept_connection(stream: TcpStream, mut receiver: Receiver<String>)  {
    let _addr = stream.peer_addr().expect("stream should have a address");
    // info!("Peer Address {}",addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("error during the ws handshake");
    // info!("New Web Socket connection: {}", addr);

    let (mut write, _) = ws_stream.split();
        while receiver.changed().await.is_ok() {
            let y: String;
            {
                let x = receiver.borrow();
                y = x.to_owned();
            }
     
            println!("received = {:?}", y);
            let msg = tokio_tungstenite::tungstenite::Message::Text(y.to_string());
            write.send(msg).await.expect("failed to send message");
        }
}



struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }
    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume(brokers: &str, group_id: &str, topics: &[&str],  sender: Sender<String>)  {
    let context = CustomContext;
    let  consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");
    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let mut message_stream =  consumer.stream();

    while let Some(message)  = message_stream.next().await {
        match message {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => {

                        sender.send(s.to_string()).expect("failed to send message internally");
                        s
                    },
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };


                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                         m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}