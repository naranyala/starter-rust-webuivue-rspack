// Persistence module for database operations
pub mod database_manager;

// Re-export the DatabaseManager struct
pub use database_manager::DatabaseManager as DatabaseManagerStruct;