use tokio::net::{TcpListener};
use tokio::sync::watch;

mod websocket;
mod kafka;
mod config;
mod cache;
mod Trajectory;

use crate::websocket::accept_connection;
use crate::kafka::{consume, consume_with_scheme};
use crate::config::{Config, handle_config};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config: Config = handle_config();
    let addr = format!("{}:{}",config.ws_host, config.ws_port);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Websocket listening on: {}", addr);
    let (tx, rx) = watch::channel("init".to_string());

    let rt = tokio::runtime::Runtime::new().unwrap();

    let kafka_task =  async move {
        let kafka_broker = format!("{}:{}", config.address, config.port);
        let topics = vec![config.topic.as_str()];
        if config.use_avro_schema {
            consume_with_scheme(kafka_broker.as_str(), config.group_id.as_str(), &topics, config.schema_registry.as_str(), tx).await;
        } else {
            consume(kafka_broker.as_str(), config.group_id.as_str(), &topics, tx).await;
        }

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


