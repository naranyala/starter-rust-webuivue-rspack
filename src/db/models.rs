use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl User {
    #[allow(dead_code)]
    pub fn new(name: String, email: String, role: String, status: String) -> Self {
        User {
            id: None,
            name,
            email,
            role,
            status,
            created_at: None,
            updated_at: None,
        }
    }
}
