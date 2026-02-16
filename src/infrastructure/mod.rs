// lib/mod.rs
// Infrastructure and shared utilities

pub mod config;
pub mod persistence;
pub mod logging;
pub mod websocket;

pub use config::config::AppConfig;
pub use persistence::DatabaseManagerStruct as DatabaseManager;
pub use logging::logging::init_logging_with_config;
