use futures::*;
use rdkafka::{ClientContext, Message, TopicPartitionList};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use tokio::sync::watch::{Sender};

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    // this callback is not used
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }
    // this callback is not used
    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }
    // this callback is not used
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

/// Subscribes to a broker with a given group Id and Topic and sends it to a given sender


pub async fn consume(broker: &str, group_id: &str, topics: &[&str], sender: Sender<String>)  {
    let context = CustomContext;
    let  consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", broker)
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
                match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => {
                        // println!("{}",s.clone());
                        sender.send(s.to_string()).expect("failed to send message internally");
                        s
                    },
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}