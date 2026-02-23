use log::{error, info, warn};
use std::sync::Arc;
use std::panic;
use webui_rs::webui;

mod core;
mod commands;
mod db;
mod infrastructure;
mod plugins;

use core::PluginManager;
use infrastructure::{AppConfig, init_logging_with_config};
use infrastructure::websocket::{WebSocketServer, get_available_port};
use plugins::{DatabasePlugin, SystemInfoPlugin, WindowTrackingPlugin};
use db::manager::DatabaseManager;
use commands::setup_all_handlers;

include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column())).unwrap_or_else(|| "unknown".to_string());
        
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        
        eprintln!("=============================================");
        eprintln!("PANIC OCCURRED!");
        eprintln!("Location: {}", location);
        eprintln!("Message: {}", message);
        eprintln!("=============================================");
        
        if let Some(backtrace) = std::env::var_os("RUST_BACKTRACE") {
            if backtrace == "1" || backtrace == "full" {
                eprintln!("Backtrace:\n{:?}", std::backtrace::Backtrace::capture());
            }
        }
    }));
}

#[tokio::main]
async fn main() {
    setup_panic_handler();
    
    let config = match AppConfig::load() {
        Ok(config) => {
            println!("Configuration loaded successfully!");
            println!("Application: {} v{}", config.get_app_name(), config.get_version());
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
    info!("Starting: {} v{}", config.get_app_name(), config.get_version());
    info!("=============================================");

    let ws_port = get_available_port().await.unwrap_or(9876);

    info!("=============================================");
    info!("Backend-Frontend Communication Options:");
    info!("");
    info!("[Transport Layer]:");
    info!("  1. WebUI IPC    - Direct IPC via WebUI library");
    info!("  2. WebSocket   - TCP-based real-time (port {})", ws_port);
    info!("  3. HTTP/REST   - Can be added for REST API");
    info!("");
    info!("[Serialization Format]:");
    info!("  1. JSON         - Human-readable (current)");
    info!("  2. MessagePack - Binary, compact");
    info!("  3. CBOR        - Binary, self-describing");
    info!("=============================================");
    info!("Selected: WebUI IPC + WebSocket (hybrid)");
    info!("  - Transport: WebUI IPC + WebSocket");
    info!("  - Serialization: JSON (serde_json)");
    info!("=============================================");

    info!("WebSocket server will run on port {}", ws_port);

    let _db_manager = match DatabaseManager::new(config.db_path.clone()) {
        Ok(manager) => Arc::new(manager),
        Err(e) => {
            error!("Failed to create database manager: {}", e);
            error!("Database path: {}", config.db_path);
            error!("Application cannot start without database");
            return;
        }
    };

    let ws_server = WebSocketServer::new(ws_port);
    if let Err(e) = ws_server.start().await {
        error!("Failed to start WebSocket server: {}", e);
        error!("WebSocket functionality will be unavailable");
        warn!("Application will continue without WebSocket support");
    } else {
        info!("WebSocket server initialized on port {}", ws_port);
    }

    // Initialize plugin manager
    let mut plugin_manager = PluginManager::new();

    // Register plugins
    plugin_manager.register(Box::new(SystemInfoPlugin::new()));
    plugin_manager.register(Box::new(DatabasePlugin::new()));
    plugin_manager.register(Box::new(WindowTrackingPlugin::new()));

    // Initialize all plugins
    if let Err(e) = plugin_manager.init_all() {
        error!("Failed to initialize plugins: {}", e);
        for plugin in ["SystemInfoPlugin", "DatabasePlugin", "WindowTrackingPlugin"] {
            warn!("Plugin '{}' may not be available", plugin);
        }
    }

    // Create window and register plugins
    let mut my_window = webui::Window::new();

    // Setup all handlers from commands module
    setup_all_handlers(&mut my_window);

    // Register all plugins
    plugin_manager.register_all(&mut my_window);

    info!("Loading application UI from frontend/dist/index.html");
    my_window.show("frontend/dist/index.html");

    info!("Application started successfully, waiting for events...");
    info!("=============================================");

    webui::wait();

    info!("Application shutting down...");
    info!("=============================================");
}
