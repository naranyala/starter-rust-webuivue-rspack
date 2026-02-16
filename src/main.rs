use log::{error, info};
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use webui_rs::webui;

static WS_PORT_STORAGE: AtomicU16 = AtomicU16::new(0);

mod core;
mod db;
mod infrastructure;
mod plugins;

use core::{AppError, Plugin, PluginManager};
use infrastructure::{AppConfig, init_logging_with_config};
use infrastructure::websocket::{WebSocketServer, get_available_port};
use plugins::{DatabasePlugin, SystemInfoPlugin, WindowTrackingPlugin};
use db::manager::DatabaseManager;
use db::models::User;

include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

fn setup_ui_handlers(window: &mut webui::Window) {
    info!("Setting up UI handlers...");

    window.bind("open_folder", |_event| {
        info!("Open folder button clicked!");
    });

    window.bind("organize_images", |_event| {
        info!("Organize images button clicked!");
    });

    window.bind("ping_backend", |event| {
        let win_id = event.window;
        info!("Ping received from window {}", win_id);

        let response = serde_json::json!({
            "success": true,
            "message": "pong",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "server": "rust-backend"
        });

        let js_code = format!(
            "if(window._webui_pong) window._webui_pong({});",
            response.to_string()
        );

        let win = webui::Window::from_id(win_id as usize);
        let _ = win.run_js(&js_code);
    });

    info!("UI handlers set up successfully");
}

fn setup_counter_handlers(window: &mut webui::Window) {
    window.bind("increment_counter", |event| {
        let element_name = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };
        info!("Counter incremented in Rust backend - Element: {}", element_name);
    });

    window.bind("reset_counter", |event| {
        let element_name = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };
        info!("Counter reset in Rust backend - Element: {}", element_name);
    });
}

#[tokio::main]
async fn main() {
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

    let db_manager = match DatabaseManager::new(config.db_path.clone()) {
        Ok(manager) => Arc::new(manager),
        Err(e) => {
            error!("Failed to create database manager: {}", e);
            return;
        }
    };

    let ws_server = WebSocketServer::new(ws_port);
    if let Err(e) = ws_server.start().await {
        error!("Failed to start WebSocket server: {}", e);
    }
    info!("WebSocket server initialized on port {}", ws_port);

    // Initialize plugin manager
    let mut plugin_manager = PluginManager::new();

    // Register plugins
    plugin_manager.register(Box::new(SystemInfoPlugin::new()));
    plugin_manager.register(Box::new(DatabasePlugin::new()));
    plugin_manager.register(Box::new(WindowTrackingPlugin::new()));

    // Initialize all plugins
    if let Err(e) = plugin_manager.init_all() {
        error!("Failed to initialize plugins: {}", e);
    }

    // Create window and register plugins
    let mut my_window = webui::Window::new();

    // Setup basic handlers
    setup_ui_handlers(&mut my_window);
    setup_counter_handlers(&mut my_window);

    // Register all plugins
    plugin_manager.register_all(&mut my_window);

    // Store WebSocket port
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
