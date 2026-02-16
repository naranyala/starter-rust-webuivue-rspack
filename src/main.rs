use log::{error, info};
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use webui_rs::webui;

static WS_PORT_STORAGE: AtomicU16 = AtomicU16::new(0);

mod commands;
mod db;
mod infrastructure;

use infrastructure::{AppConfig, init_logging_with_config};
use infrastructure::websocket::{WebSocketServer, get_available_port};
use db::manager::DatabaseManager;

include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

#[tokio::main]
async fn main() {
    let config = match AppConfig::load() {
        Ok(config) => {
            println!("Configuration loaded successfully!");
            println!(
                "Application: {} v{}",
                config.get_app_name(),
                config.get_version()
            );
            config
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("Using default configuration");
            AppConfig::default()
        }
    };

    if let Err(e) = init_logging_with_config(
        Some(&config.log_file),
        config.get_log_level(),
        config.is_append_log(),
    ) {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }

    info!("=============================================");
    info!(
        "Starting: {} v{}",
        config.get_app_name(),
        config.get_version()
    );
    info!("=============================================");

    info!("Application starting...");

    let ws_port = get_available_port().await.unwrap_or(9876);

    info!("=============================================");
    info!("Backend-Frontend Communication Options:");
    info!("");
    info!("[Transport Layer]:");
    info!("  1. WebUI IPC    - Direct IPC via WebUI library");
    info!("  2. WebSocket    - TCP-based real-time (port {})", ws_port);
    info!("  3. HTTP/REST    - Can be added for REST API");
    info!("");
    info!("[Serialization Format]:");
    info!("  1. JSON         - Human-readable (current)");
    info!("  2. MessagePack  - Binary, compact");
    info!("  3. CBOR         - Binary, self-describing");
    info!("  4. BinCode      - Binary, Rust-specific");
    info!("  5. Postcard     - Binary, no-std");
    info!("=============================================");
    info!("Selected: WebUI IPC + WebSocket (hybrid)");
    info!("  - Transport: WebUI IPC + WebSocket");
    info!("  - Serialization: JSON (serde_json)");
    info!("=============================================");

    info!("WebSocket server will run on port {}", ws_port);

    let db_manager = Arc::new(
        DatabaseManager::new(config.db_path.clone())
            .expect("Failed to create database manager")
    );

    if config.should_create_sample_data() {
        db_manager.insert_sample_data().ok();
    }

    let ws_server = WebSocketServer::new(ws_port);
    if let Err(e) = ws_server.start().await {
        error!("Failed to start WebSocket server: {}", e);
    }
    info!("WebSocket server initialized on port {}", ws_port);
    info!("WebSocket server running on port {}", ws_port);

    let mut my_window = webui::Window::new();

    commands::setup_all_handlers(&mut my_window);

    WS_PORT_STORAGE.store(ws_port, Ordering::Relaxed);
    my_window.bind("get_port_info", get_port_info_handler);

    info!("Loading application UI from frontend/dist/index.html");
    my_window.show("frontend/dist/index.html");

    info!("Application started successfully, waiting for events...");
    info!("=============================================");

    webui::wait();

    info!("Application shutting down...");
    info!("=============================================");
}

fn get_port_info_handler(event: webui::Event) {
    let port = WS_PORT_STORAGE.load(Ordering::Relaxed) as i64;
    
    let js = format!(
        "window._webui_port_callback && window._webui_port_callback({})",
        port
    );
    
    webui::Window::from_id(event.window).run_js(&js);
}
