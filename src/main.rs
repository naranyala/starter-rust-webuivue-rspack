use log::{error, info};
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use webui_rs::webui;

// Global storage for WebSocket port
static WS_PORT_STORAGE: AtomicU16 = AtomicU16::new(0);

// Import from new MVVM layers
mod domains;
mod application;
mod infrastructure;
mod presentation;
mod shared;

use crate::infrastructure::{AppConfig, init_logging_with_config};
use crate::infrastructure::websocket::{WebSocketServer, get_available_port};

// Build-time generated config
include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

#[tokio::main]
async fn main() {
    // Load application configuration
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

    // Initialize logging system with config settings
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

    // Get available port for WebSocket server
    let ws_port = get_available_port().await.unwrap_or(9876);
    info!("WebSocket server will run on port {}", ws_port);

    // Initialize database
    let db_manager = Arc::new(
        crate::infrastructure::DatabaseManager::new(config.db_path.clone())
            .expect("Failed to create database manager")
    );

    if config.should_create_sample_data() {
        db_manager.insert_sample_data().ok();
    }

    // Create WebSocket server
    let ws_server = WebSocketServer::new(ws_port);
    if let Err(e) = ws_server.start().await {
        error!("Failed to start WebSocket server: {}", e);
    }
    info!("WebSocket server initialized on port {}", ws_port);

    // Pass WebSocket port to frontend via JavaScript
    info!("WebSocket server running on port {}", ws_port);

    // Create a new window
    let mut my_window = webui::Window::new();

    // Set up presentation layer handlers
    presentation::handlers::ui_handlers::setup_ui_handlers(&mut my_window);
    presentation::handlers::counter_handlers::setup_counter_handlers(&mut my_window);
    presentation::handlers::sysinfo_handlers::setup_sysinfo_handlers(&mut my_window);
    presentation::handlers::window_state_handlers::setup_window_state_handlers(&mut my_window);

    // Store the WebSocket port globally so it can be accessed by the handler
    WS_PORT_STORAGE.store(ws_port, Ordering::Relaxed);

    // Bind the WebSocket port to the frontend
    my_window.bind("get_port_info", get_port_info_handler);

    // Show the built Vue.js application
    info!("Loading application UI from frontend/dist/index.html");
    my_window.show("frontend/dist/index.html");

    info!("Application started successfully, waiting for events...");
    info!("=============================================");

    // Wait until all windows are closed
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
