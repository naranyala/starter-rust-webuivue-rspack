use crate::core::error::{lock_error, AppError, AppResult};
use crate::db::models::User;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct DatabaseManager {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    pub fn new(db_path: String) -> AppResult<Self> {
        let conn = Connection::open(&db_path).map_err(AppError::Database)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(AppError::Database)?;

        let conn = Arc::new(Mutex::new(conn));
        let db_manager = Self { conn };

        db_manager.init()?;
        Ok(db_manager)
    }

    pub fn init(&self) -> AppResult<()> {
        let conn = self.conn.lock().map_err(lock_error)?;

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
        )
        .map_err(AppError::Database)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn insert_sample_data(&self) -> AppResult<()> {
        let conn = self.conn.lock().map_err(lock_error)?;

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
            .map_err(AppError::Database)?;

        if count == 0 {
            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["Admin User", "admin@example.com", "Administrator", "Active"],
            )
            .map_err(AppError::Database)?;

            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["John Doe", "john@example.com", "User", "Active"],
            )
            .map_err(AppError::Database)?;

            conn.execute(
                "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
                params!["Jane Smith", "jane@example.com", "User", "Active"],
            )
            .map_err(AppError::Database)?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_all_users(&self) -> AppResult<Vec<User>> {
        let conn = self.conn.lock().map_err(lock_error)?;

        let mut stmt = conn.prepare(
            "SELECT id, name, email, role, status, created_at, updated_at FROM users ORDER BY id",
        ).map_err(AppError::Database)?;

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
            })
            .map_err(AppError::Database)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(AppError::Database)?;

        Ok(users)
    }

    #[allow(dead_code)]
    pub fn get_user_by_id(&self, id: i64) -> AppResult<Option<User>> {
        let conn = self.conn.lock().map_err(lock_error)?;

        let mut stmt = conn.prepare(
            "SELECT id, name, email, role, status, created_at, updated_at FROM users WHERE id = ?1",
        ).map_err(AppError::Database)?;

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
            Err(e) => Err(AppError::Database(e)),
        }
    }

    #[allow(dead_code)]
    pub fn insert_user(&self, name: &str, email: &str, role: &str, status: &str) -> AppResult<i64> {
        let conn = self.conn.lock().map_err(lock_error)?;

        conn.execute(
            "INSERT INTO users (name, email, role, status) VALUES (?1, ?2, ?3, ?4)",
            params![name, email, role, status],
        )
        .map_err(AppError::Database)?;

        Ok(conn.last_insert_rowid())
    }

    #[allow(dead_code)]
    pub fn update_user(
        &self,
        id: i64,
        name: Option<&str>,
        email: Option<&str>,
        role: Option<&str>,
        status: Option<&str>,
    ) -> AppResult<usize> {
        let conn = self.conn.lock().map_err(lock_error)?;

        let mut set_parts = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name_val) = name {
            set_parts.push(format!("name = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(name_val.to_string()));
        }

        if let Some(email_val) = email {
            set_parts.push(format!("email = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(email_val.to_string()));
        }

        if let Some(role_val) = role {
            set_parts.push(format!("role = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(role_val.to_string()));
        }

        if let Some(status_val) = status {
            set_parts.push(format!("status = ?{}", params_vec.len() + 1));
            params_vec.push(Box::new(status_val.to_string()));
        }

        if set_parts.is_empty() {
            return Ok(0);
        }

        params_vec.push(Box::new(id));

        let sql = format!(
            "UPDATE users SET {} WHERE id = ?{}",
            set_parts.join(", "),
            params_vec.len()
        );

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref()).collect();

        let affected_rows = conn
            .execute(&sql, params_refs.as_slice())
            .map_err(AppError::Database)?;

        Ok(affected_rows)
    }

    #[allow(dead_code)]
    pub fn delete_user(&self, id: i64) -> AppResult<usize> {
        let conn = self.conn.lock().map_err(lock_error)?;

        let affected_rows = conn
            .execute("DELETE FROM users WHERE id = ?1", [id])
            .map_err(AppError::Database)?;

        Ok(affected_rows)
    }
}
