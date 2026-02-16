use crate::domains::user::entity::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserService {
    async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>>;
    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn create_user(&self, user: User) -> Result<i64, Box<dyn std::error::Error>>;
    async fn update_user(&self, user: User) -> Result<bool, Box<dyn std::error::Error>>;
    async fn delete_user(&self, id: i64) -> Result<bool, Box<dyn std::error::Error>>;
}

use std::sync::Arc;

pub struct DefaultUserService {
    db_manager: Arc<crate::infrastructure::DatabaseManager>,
}

impl DefaultUserService {
    pub fn new(db_manager: Arc<crate::infrastructure::DatabaseManager>) -> Self {
        Self { db_manager }
    }
}

#[async_trait]
impl UserService for DefaultUserService {
    async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        // Delegate to the database manager
        let users = self.db_manager.get_all_users()?;
        Ok(users.into_iter().map(|u| User {
            id: u.id,
            name: u.name,
            email: u.email,
            role: u.role,
            status: u.status,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }).collect())
    }

    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn std::error::Error>> {
        // Delegate to the database manager
        match self.db_manager.get_user_by_id(id)? {
            Some(user) => Ok(Some(User {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
                status: user.status,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })),
            None => Ok(None),
        }
    }

    async fn create_user(&self, user: User) -> Result<i64, Box<dyn std::error::Error>> {
        // Convert User to the database representation and delegate
        let id = self.db_manager.insert_user(&user.name, &user.email, &user.role, &user.status)?;
        Ok(id)
    }

    async fn update_user(&self, user: User) -> Result<bool, Box<dyn std::error::Error>> {
        // Delegate to the database manager
        if let Some(id) = user.id {
            let rows_updated = self.db_manager.update_user(id, Some(&user.name), Some(&user.email), Some(&user.role), Some(&user.status))?;
            Ok(rows_updated > 0)
        } else {
            Err("User ID is required for update".into())
        }
    }

    async fn delete_user(&self, id: i64) -> Result<bool, Box<dyn std::error::Error>> {
        // Delegate to the database manager
        let rows_deleted = self.db_manager.delete_user(id)?;
        Ok(rows_deleted > 0)
    }
}