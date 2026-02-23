pub mod error;
pub mod plugin;

#[allow(unused_imports)]
pub use error::AppError;
pub use plugin::{Plugin, PluginManager};
