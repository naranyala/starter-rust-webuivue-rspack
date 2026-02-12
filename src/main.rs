use log::info;
use std::sync::Arc;
use webui_rs::webui;

// Import from infrastructure layer
mod infrastructure;
use infrastructure::{config::AppConfig, database::Database, di, logging};

// Import from utilities layer
mod utilities;

// Import from use cases layer (business logic)
mod use_cases;
use use_cases::handlers;

// Build-time generated config
include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

fn main() {
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
    if let Err(e) = logging::init_logging_with_config(
        Some(config.get_log_file()),
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

    // Initialize dependency injection container
    di::init_container();
    info!("Dependency injection container initialized");

    info!("Application starting...");

    // Get database path from config
    let db_path = config.get_db_path();
    info!("Database path: {}", db_path);

    // Initialize SQLite database
    let db = match Database::new(db_path) {
        Ok(db) => {
            info!("Database initialized successfully");
            if let Err(e) = db.init() {
                eprintln!("Failed to initialize database schema: {}", e);
                return;
            }
            if config.should_create_sample_data() {
                if let Err(e) = db.insert_sample_data() {
                    eprintln!("Failed to insert sample data: {}", e);
                    return;
                }
                info!("Sample data created (if not exists)");
            }
            Arc::new(db)
        }
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    // Initialize database handlers with the database instance
    handlers::db_handlers::init_database(Arc::clone(&db));

    // Create a new window
    let mut my_window = webui::Window::new();

    // Set up UI event handlers from use-cases
    handlers::ui_handlers::setup_ui_handlers(&mut my_window);
    handlers::ui_handlers::setup_counter_handlers(&mut my_window);
    handlers::db_handlers::setup_db_handlers(&mut my_window);
    handlers::sysinfo_handlers::setup_sysinfo_handlers(&mut my_window);
    handlers::utilities_handlers::setup_utilities_handlers(&mut my_window);

    // Get window settings from config
    let window_title = config.get_window_title();
    info!("Window title: {}", window_title);

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
