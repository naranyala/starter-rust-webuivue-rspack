use crate::domains::user::entity::User;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct DatabaseManager {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    pub fn new(db_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(&db_path)?;
        
        // Enable foreign key constraints
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        
        let conn = Arc::new(Mutex::new(conn));
        let db_manager = Self { conn };
        
        db_manager.init()?;
        Ok(db_manager)
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        // Create users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                role TEXT DEFAULT 'User',
                status TEXT DEFAULT 'Active',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(())
    }

    pub fn insert_sample_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        // Check if users already exist
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        
        if count == 0 {
            // Insert sample users
            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["Admin User", "admin@example.com", "Administrator", "Active"],
            )?;
            
            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["John Doe", "john@example.com", "User", "Active"],
            )?;
            
            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["Jane Smith", "jane@example.com", "User", "Active"],
            )?;
        }
        
        Ok(())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, name, email, role, status, created_at, updated_at FROM users ORDER BY id",
        )?;
        
        let users = stmt
            .query_map([], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    email: row.get(2)?,
                    role: row.get(3)?,
                    status: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(users)
    }

    pub fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, name, email, role, status, created_at, updated_at FROM users WHERE id = ?1",
        )?;
        
        match stmt.query_row([id], |row| {
            Ok(User {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                email: row.get(2)?,
                role: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        }) {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn insert_user(&self, name: &str, email: &str, role: &str, status: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
            params![name, email, role, status],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn update_user(&self, id: i64, name: Option<&str>, email: Option<&str>, role: Option<&str>, status: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        let mut set_parts = Vec::new();
        let mut params = Vec::new();
        
        if let Some(name_val) = name {
            set_parts.push(format!("name = ?{}", params.len() + 1));
            params.push(name_val);
        }
        
        if let Some(email_val) = email {
            set_parts.push(format!("email = ?{}", params.len() + 1));
            params.push(email_val);
        }
        
        if let Some(role_val) = role {
            set_parts.push(format!("role = ?{}", params.len() + 1));
            params.push(role_val);
        }
        
        if let Some(status_val) = status {
            set_parts.push(format!("status = ?{}", params.len() + 1));
            params.push(status_val);
        }
        
        if set_parts.is_empty() {
            return Ok(0);
        }
        
        let id_str = id.to_string(); // Create a longer-lived value
        params.push(&id_str); // For WHERE clause
        
        let sql = format!(
            "UPDATE users SET {} WHERE id = ?{}",
            set_parts.join(", "),
            params.len()
        );
        
        let affected_rows = conn.execute(&sql, rusqlite::params_from_iter(params))?;
        Ok(affected_rows)
    }

    pub fn delete_user(&self, id: i64) -> Result<usize, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        
        let affected_rows = conn.execute("DELETE FROM users WHERE id = ?1", [id])?;
        Ok(affected_rows)
    }
}