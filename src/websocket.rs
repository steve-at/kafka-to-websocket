use futures::*;
use tokio::net::{TcpStream};
use tokio::sync::watch::{Receiver};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::cache::Cache;
use crate::Trajectory::Trajectory;


/// takes an TcpStream and a Receiver
///
/// the Receiver is from the messaging service like Kafka (the one of the sender which is passed
/// to the kafka consumer
///
///
/// The TCP Stream is the TCP Stream from the listening Method from the TCPListener
pub async fn accept_connection(stream: TcpStream, mut receiver: Receiver<String>) {
    let _addr = stream.peer_addr().expect("stream should have a address");
    let mut cache = Cache::new();
    let mut buffer: Vec<Trajectory> = vec![];
    let mut connection_start = Instant::now();
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("error during the ws handshake");
    println!("New Web Socket connection: {}", &_addr);

    let (mut write, _) = ws_stream.split();
    while receiver.changed().await.is_ok() {
        let y: String;
        {
            let x = receiver.borrow();
            let track = cache.write_and_get_delta(x.as_str()).unwrap();
            buffer.push(track);
            // y = x.to_owned();
            // y = serde_json::to_string(&track).unwrap();
        }
        // to save extra messages the buffer is transmitted only every <batch_window> milliseconds
        if connection_start.elapsed().as_millis() > 250 {
            connection_start = Instant::now();
            let message_as_string = serde_json::to_string(&buffer).unwrap().to_string();
            let msg = tokio_tungstenite::tungstenite::Message::Text(message_as_string);
            match write.send(msg).await {
                Ok(_) => {
                    // println!("{:?}", &buffer);
                    buffer.clear();
                }
                Err(_) => continue, // prevents Panic when a broken pipe happens
            }
            //     remove old items from cache
            let mut to_be_removed: Vec<String> = vec![];
            for (id, entry) in cache.updated.iter() {
                if entry.clone() < (SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() - 60_000) as u64 {
                    to_be_removed.push(id.clone());
                }
            }
            // println!("{:?}",to_be_removed);
            for entry in to_be_removed.iter() {
                cache.entries.remove(entry);
                // println!("{:?}", cache.entries.keys())
            }
            for entry in to_be_removed.iter() {
                cache.updated.remove(entry);
                println!("{:?}", cache.entries.keys())
            }
        }
    }
}