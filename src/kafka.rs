use avro_rs::{from_value};
use futures::*;
use rdkafka::{ClientContext, Message, TopicPartitionList};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use schema_registry_converter::async_impl::avro::AvroDecoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use tokio::sync::watch::Sender;

use crate::custom_schema::Track;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    // this callback is not used
    fn pre_rebalance(&self, _rebalance: &Rebalance) {
        // println!("Pre rebalance {:?}", _rebalance);
    }
    // this callback is not used
    fn post_rebalance(&self, _rebalance: &Rebalance) {
        // println!("Post rebalance {:?}", _rebalance);
    }
    // this callback is not used
    fn commit_callback(&self, _result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        // println!("Committing offsets: {:?}", _result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

/// Subscribes to a broker with a given group Id and Topic and sends it to a given sender

fn setup_config_for_consumer(group_id: &str, broker: &str, context: CustomContext) -> LoggingConsumer {
    ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", broker)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Error)
        .create_with_context(context)
        .expect("Consumer creation failed")
}
pub async fn consume(broker: &str, group_id: &str, topics: &[&str], sender: Sender<String>)  {
    let context = CustomContext;
    let consumer = setup_config_for_consumer(group_id, broker, context);
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

pub async fn consume_with_scheme(broker: &str, group_id: &str, topics: &[&str], schema_registry: &str, sender: Sender<String>)  {
    let context = CustomContext;
    let sr_settings = SrSettings::new(schema_registry.to_string());
    let mut decoder = AvroDecoder::new(sr_settings);
    let consumer = setup_config_for_consumer(group_id, broker, context);
    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let mut message_stream =  consumer.stream();

    while let Some(message)  = message_stream.next().await {
        match message {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                match decoder.decode(m.payload()).await {
                    Ok(result) => {
                        match from_value::<Track>(&result.value) {
                            Ok(track) => {
                                println!("{:?}",&track);
                                match serde_json::to_string(&track) {
                                    Ok(json_string) => {
                                        sender.send(json_string).expect("failed to send message internally");
                                    }
                                    Err(_) => {println!("Failed to stringify the Message")}
                                }
                            }
                            Err(_) => { println!("received value not within the given scheme")}
                        }
                    }
                    Err(_) => {println!("error receiving")}
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}