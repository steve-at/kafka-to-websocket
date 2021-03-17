use futures::*;
use tokio::net::{TcpStream};
use tokio::sync::watch::{Receiver};


/// takes an TcpStream and a Receiver
///
/// the Receiver is from the messaging service like Kafka (the one of the sender which is passed
/// to the kafka consumer
///
/// The TCP Stream is the TCP Stream from the listening Method from the TCPListener
pub async fn accept_connection(stream: TcpStream, mut receiver: Receiver<String>)  {
    let _addr = stream.peer_addr().expect("stream should have a address");


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

        let msg = tokio_tungstenite::tungstenite::Message::Text(y.to_string());
        write.send(msg).await.expect("failed to send message");
    }
}