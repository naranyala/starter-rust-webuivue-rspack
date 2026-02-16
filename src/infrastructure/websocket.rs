use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};

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
    let (mut write, mut read) = ws_stream.split();

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
                                    let sysinfo = get_system_info_impl();
                                    WsMessage::Response {
                                        request_id,
                                        success: true,
                                        data: Some(serde_json::json!(sysinfo)),
                                        error: None,
                                    }
                                }
                                "get_uptime" => {
                                    let uptime_str = get_uptime_impl();
                                    let uptime_seconds = parse_uptime_to_seconds_impl(&uptime_str);
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

// Copy the system info functions from sysinfo_handlers.rs
fn get_system_info_impl() -> serde_json::Value {
    let mut sysinfo = serde_json::Map::new();

    sysinfo.insert(
        "os".to_string(),
        serde_json::json!({
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "family": std::env::consts::FAMILY,
        }),
    );

    let mem_info = get_memory_info_impl();
    sysinfo.insert("memory".to_string(), mem_info);

    let cpu_info = get_cpu_info_impl();
    sysinfo.insert("cpu".to_string(), cpu_info);

    let disk_info = get_disk_info_impl();
    sysinfo.insert("disk".to_string(), disk_info);

    let uptime = get_uptime_impl();
    sysinfo.insert("uptime".to_string(), serde_json::json!(uptime));

    sysinfo.insert(
        "env_vars".to_string(),
        serde_json::json!(std::env::vars_os().count()),
    );

    let current_dir = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    sysinfo.insert("cwd".to_string(), serde_json::json!(current_dir));

    serde_json::Value::Object(sysinfo)
}

fn get_memory_info_impl() -> serde_json::Value {
    let mut mem = serde_json::Map::new();

    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().split_whitespace().next();

                match key {
                    "MemTotal" => {
                        mem.insert(
                            "total_mb".to_string(),
                            serde_json::json!(parse_mem_value_impl(value)),
                        );
                    }
                    "MemFree" => {
                        mem.insert(
                            "free_mb".to_string(),
                            serde_json::json!(parse_mem_value_impl(value)),
                        );
                    }
                    "MemAvailable" => {
                        mem.insert(
                            "available_mb".to_string(),
                            serde_json::json!(parse_mem_value_impl(value)),
                        );
                    }
                    "Buffers" => {
                        mem.insert(
                            "buffers_mb".to_string(),
                            serde_json::json!(parse_mem_value_impl(value)),
                        );
                    }
                    "Cached" => {
                        mem.insert(
                            "cached_mb".to_string(),
                            serde_json::json!(parse_mem_value_impl(value)),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    serde_json::Value::Object(mem)
}

fn parse_mem_value_impl(value: Option<&str>) -> f64 {
    match value {
        Some(v) => v.parse::<u64>().unwrap_or(0) as f64 / 1024.0,
        None => 0.0,
    }
}

fn get_cpu_info_impl() -> serde_json::Value {
    let mut cpu = serde_json::Map::new();

    if let Ok(count) = std::fs::read_to_string("/proc/cpuinfo") {
        let core_count = count.lines().filter(|l| l.starts_with("processor")).count();
        cpu.insert("cores".to_string(), serde_json::json!(core_count));
    }

    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("Model") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    cpu.insert("model".to_string(), serde_json::json!(parts[1].trim()));
                    break;
                }
            }
        }
    }

    cpu.insert(
        "usage_percent".to_string(),
        serde_json::json!(get_cpu_usage_impl()),
    );

    serde_json::Value::Object(cpu)
}

fn get_cpu_usage_impl() -> f64 {
    if let Ok(content) = std::fs::read_to_string("/proc/stat") {
        let lines: Vec<&str> = content.lines().collect();
        if let Some(first_line) = lines.first() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 8 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total = user + system + idle;

                if total > 0 {
                    return ((user + system) as f64 / total as f64) * 100.0;
                }
            }
        }
    }
    0.0
}

fn get_disk_info_impl() -> serde_json::Value {
    let mut disks = Vec::new();

    if let Ok(output) = std::process::Command::new("df")
        .args(&["-h", "-P", "-x", "tmpfs", "-x", "devtmpfs"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    disks.push(serde_json::json!({
                        "filesystem": parts[0],
                        "size": parts[1],
                        "used": parts[2],
                        "available": parts[3],
                        "use_percent": parts[4],
                        "mount": parts[5],
                    }));
                }
            }
        }
    }

    serde_json::Value::Array(disks)
}

fn get_uptime_impl() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if let Some(uptime_str) = parts.first() {
            if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                let days = (uptime_secs / 86400.0) as u64;
                let hours = ((uptime_secs % 86400.0) / 3600.0) as u64;
                let minutes = ((uptime_secs % 3600.0) / 60.0) as u64;
                let seconds = (uptime_secs % 60.0) as u64;

                return format!("{}d {}h {}m {}s", days, hours, minutes, seconds);
            }
        }
    }
    "unknown".to_string()
}

fn parse_uptime_to_seconds_impl(uptime_str: &str) -> u64 {
    if uptime_str == "unknown" {
        return 0;
    }
    
    let mut total_seconds = 0;
    
    // Split by spaces and parse each part
    let parts: Vec<&str> = uptime_str.split_whitespace().collect();
    for part in parts {
        if part.ends_with('d') {
            if let Ok(days) = part.trim_end_matches('d').parse::<u64>() {
                total_seconds += days * 86400;
            }
        } else if part.ends_with('h') {
            if let Ok(hours) = part.trim_end_matches('h').parse::<u64>() {
                total_seconds += hours * 3600;
            }
        } else if part.ends_with('m') {
            if let Ok(minutes) = part.trim_end_matches('m').parse::<u64>() {
                total_seconds += minutes * 60;
            }
        } else if part.ends_with('s') {
            if let Ok(seconds) = part.trim_end_matches('s').parse::<u64>() {
                total_seconds += seconds;
            }
        }
    }
    
    total_seconds
}

pub async fn get_available_port() -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    Ok(addr.port())
}