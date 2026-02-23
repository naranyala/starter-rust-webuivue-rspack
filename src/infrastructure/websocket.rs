use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use super::{get_system_info, get_uptime, parse_uptime_to_seconds};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "ping")]
    Ping { timestamp: i64 },
    #[serde(rename = "pong")]
    Pong { timestamp: i64 },
    #[serde(rename = "message")]
    Message { data: String },
    #[serde(rename = "broadcast")]
    Broadcast { data: String },
    #[serde(rename = "request")]
    Request { request_type: String, request_id: String, data: Option<serde_json::Value> },
    #[serde(rename = "response")]
    Response { request_id: String, success: bool, data: Option<serde_json::Value>, error: Option<String> },
}

#[derive(Clone)]
pub struct WebSocketServer {
    clients: Arc<RwLock<Vec<broadcast::Sender<WsMessage>>>>,
    port: u16,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Self {
        Self {
            clients: Arc::new(RwLock::new(Vec::new())),
            port,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("WebSocket server listening on ws://{}", addr);

        let clients = self.clients.clone();

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                let clients = clients.clone();
                tokio::spawn(handle_connection(stream, addr, clients));
            }
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Arc<RwLock<Vec<broadcast::Sender<WsMessage>>>>,
) {
    info!("New WebSocket connection from: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let (write_tx, mut write_rx) = mpsc::unbounded_channel::<String>();
    let (write, mut read) = ws_stream.split();

    let (tx, _rx) = broadcast::channel::<WsMessage>(100);
    let tx_clone = tx.clone();

    {
        let mut clients_guard = clients.write().await;
        clients_guard.push(tx_clone);
    }

    // Handle outgoing messages from broadcast
    let tx_out = tx.clone();
    let write_tx_clone = write_tx.clone();
    tokio::spawn(async move {
        let mut rx = tx_out.subscribe();
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = write_tx_clone.send(json);
            }
        }
    });

    // Handle sending to WebSocket
    let mut write_sink = write;
    tokio::spawn(async move {
        while let Some(response) = write_rx.recv().await {
            if write_sink.send(Message::Text(response.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let tx_for_requests = tx.clone();
    let write_tx_for_requests = write_tx.clone();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    match ws_msg {
                        WsMessage::Ping { timestamp } => {
                            let pong = WsMessage::Pong { timestamp };
                            let _ = tx_for_requests.send(pong);
                        }
                        WsMessage::Message { data } => {
                            info!("Received message from {}: {}", addr, data);
                            let _ = tx_for_requests.send(WsMessage::Message { data });
                        }
                        WsMessage::Broadcast { data } => {
                            info!("Received broadcast from {}: {}", addr, data);
                            // Forward to all clients
                            let broadcast_msg = WsMessage::Broadcast { data: data.clone() };
                            let clients_guard = clients.read().await;
                            for client_tx in clients_guard.iter() {
                                let _ = client_tx.send(broadcast_msg.clone());
                            }
                        }
                        WsMessage::Request { request_type, request_id, data: _ } => {
                            // Handle specific requests
                            let response = match request_type.as_str() {
                                "get_system_info" => {
                                    let sysinfo = get_system_info();
                                    WsMessage::Response {
                                        request_id,
                                        success: true,
                                        data: Some(serde_json::json!(sysinfo)),
                                        error: None,
                                    }
                                }
                                "get_uptime" => {
                                    let uptime_str = get_uptime();
                                    let uptime_seconds = parse_uptime_to_seconds(&uptime_str);
                                    WsMessage::Response {
                                        request_id,
                                        success: true,
                                        data: Some(serde_json::json!({"uptime_seconds": uptime_seconds})),
                                        error: None,
                                    }
                                }
                                _ => {
                                    WsMessage::Response {
                                        request_id,
                                        success: false,
                                        data: None,
                                        error: Some(format!("Unknown request type: {}", request_type)),
                                    }
                                }
                            };
                            
                            // Send response directly through the write channel
                            if let Ok(response_text) = serde_json::to_string(&response) {
                                let _ = write_tx_for_requests.send(response_text);
                            }
                        }
                        _ => {
                            // For other message types, just forward them
                            let _ = tx_for_requests.send(ws_msg);
                        }
                    }
                } else {
                    error!("Invalid message format from {}: {}", addr, text);
                }
            }
            Ok(Message::Binary(_)) => {
                // Ignore binary messages
                info!("Ignoring binary message from {}", addr);
            }
            Ok(Message::Ping(_)) => {
                // Tungstenite handles ping/pong automatically
            }
            Ok(Message::Pong(_)) => {
                // Tungstenite handles ping/pong automatically
            }
            Ok(Message::Frame(_)) => {
                // Ignore frame messages
                info!("Ignoring frame message from {}", addr);
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket connection closed: {}", addr);
                break;
            }
            Err(e) => {
                error!("WebSocket error from {}: {}", addr, e);
                break;
            }
        }
    }

    // Client cleanup happens via sender.drop() when connection closes
    info!("WebSocket connection removed: {}", addr);
}

pub async fn get_available_port() -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    Ok(addr.port())
}