use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, prelude::Type};
use std::fmt::Display;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[sqlx(type_name = "roles")]
pub enum UserRole {
    Admin,
    Staff,
}
impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::Staff => write!(f, "Staff"),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Deserialize, FromRow)]
pub struct LoginChallenge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
}
