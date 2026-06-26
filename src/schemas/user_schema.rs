use crate::models::user::UserRole;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub user: RegisterUserData,
}
#[derive(Debug, Deserialize, FromRow)]
pub struct RegisterUserData {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}
