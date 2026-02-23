use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_executable_name")]
    pub executable_name: String,

    #[serde(default = "default_db_path")]
    pub db_path: String,

    #[serde(default = "default_create_sample_data")]
    pub create_sample_data: bool,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_log_file")]
    pub log_file: String,
    
    #[serde(default = "default_append_log")]
    pub append_log: bool,
    
    #[serde(default = "default_window_title")]
    pub window_title: String,
}

fn default_executable_name() -> String {
    "app".to_string()
}

fn default_db_path() -> String {
    "app.db".to_string()
}

fn default_create_sample_data() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_file() -> String {
    "application.log".to_string()
}

fn default_append_log() -> bool {
    true
}

fn default_window_title() -> String {
    "Rust WebUI Application".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            executable_name: default_executable_name(),
            db_path: default_db_path(),
            create_sample_data: default_create_sample_data(),
            log_level: default_log_level(),
            log_file: default_log_file(),
            append_log: default_append_log(),
            window_title: default_window_title(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("app.config.toml")?;
        let config: AppConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    #[allow(dead_code)]
    pub fn should_create_sample_data(&self) -> bool {
        self.create_sample_data
    }

    pub fn get_log_level(&self) -> &str {
        &self.log_level
    }

    pub fn get_app_name(&self) -> String {
        self.executable_name.clone()
    }

    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    pub fn is_append_log(&self) -> bool {
        self.append_log
    }
    
    #[allow(dead_code)]
    pub fn get_window_title(&self) -> String {
        self.window_title.clone()
    }
}