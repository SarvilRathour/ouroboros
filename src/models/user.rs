use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, prelude::Type};
use std::{cmp::max, fmt::Display};
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

#[derive(Debug, Deserialize, FromRow)]
pub struct TwoFactorLogin {
    pub id: Uuid,
    pub code: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub enum task_status {
    todo,
    in_progress,
    done,
}
impl Display for task_status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            task_status::todo => write!(f, "todo"),
            task_status::in_progress => write!(f, "in_progress"),
            task_status::done => write!(f, "done"),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub enum task_priority {
    low,
    medium,
    high,
}
impl Display for task_priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            task_priority::low => write!(f, "low"),
            task_priority::medium => write!(f, "medium"),
            task_priority::high => write!(f, "high"),
        }
    }
}
#[derive(Debug, Deserialize, FromRow)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
    pub status: task_status,
    pub priority: task_priority,
    pub assign_to_email: String,
}
