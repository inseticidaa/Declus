use uuid::Uuid;
use validator::Validate;

use crate::database::models::UserEntity;

#[derive(Debug, Validate)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct GetUser {
    pub email: Option<String>,
    pub username: Option<String>
}

#[derive(Debug, Clone)]
pub struct UserResponse {
    pub token: Option<String>,
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl From<UserEntity> for UserResponse {
    fn from(entity: UserEntity) -> Self {
        Self {
            token: None,
            id: entity.id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            username: entity.username,
        }
    }
}

impl Default for UserResponse {
    fn default() -> Self {
        Self {
            token: None,
            id: Uuid::new_v4(),
            first_name: "Default".to_string(),
            last_name: "Default".to_string(),
            email: "Default".to_string(),
            username: "Default".to_string(),
        }
    }
}