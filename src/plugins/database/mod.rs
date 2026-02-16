use crate::core::Plugin;
use crate::db::manager::DatabaseManager;
use log::info;
use std::sync::Arc;
use webui_rs::webui;

pub struct DatabasePlugin;

impl DatabasePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DatabasePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DatabasePlugin {
    fn name(&self) -> &str {
        "database"
    }

    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing DatabasePlugin...");
        Ok(())
    }

    fn register(&self, _window: &mut webui::Window) {
        info!("DatabasePlugin registered (handlers in main.rs)");
    }
}
