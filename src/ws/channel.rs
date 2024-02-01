use serde_json::json;
use std::error::Error;
use tokio::sync::mpsc;

use super::server::WebSocket;

pub struct Data {
    pub offer: String,
    pub ts: String,
}

pub async fn transmit(
    server: WebSocket,
    mut receiver: mpsc::Receiver<Data>,
) -> Result<(), Box<dyn Error>> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(300));
    let mut buffer = Vec::new();
    loop {
        tokio::select! {
                Some(data) = receiver.recv() => {
                    buffer.push(data);
                },
                _ = interval.tick() => {
                    for data in buffer.drain(..) {
                server
                    .send(
                        json!({
                        "offer": data.offer,
                        "ts": data.ts
                        })
                        .to_string(),
                    )
                    .await;
        }}}
        if false {
            break;
        }
    }
    Ok(())
}
