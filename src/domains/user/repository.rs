use super::entity::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>>;
    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn create_user(&self, user: User) -> Result<i64, Box<dyn std::error::Error>>;
    async fn update_user(&self, user: User) -> Result<bool, Box<dyn std::error::Error>>;
    async fn delete_user(&self, id: i64) -> Result<bool, Box<dyn std::error::Error>>;
}