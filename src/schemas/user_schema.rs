use crate::models::user::UserRole;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
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
#[derive(Debug, Deserialize, FromRow)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}
// #[derive(Debug, Deserialize, FromRow)]
// pub struct CreateTaskRequest{
//     pub title:String,
    
// }