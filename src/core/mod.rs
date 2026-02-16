pub mod error;
pub mod plugin;

pub use error::{AppError, AppResult};
pub use plugin::{Plugin, PluginManager, PluginRegistry};
