use uuid::Uuid;
use chrono::{DateTime,Utc};
use serde::{Serialize,Deserialize};
use sqlx::{Decode, FromRow, prelude::Type};
#[derive(Serialize, Deserialize,Debug,Clone,Type)]
#[sqlx(type_name="roles")]

pub enum UserRole{
    Admin,
    Staff
}
#[derive(Serialize, Deserialize,Debug,FromRow,Clone)]
pub struct User{
    pub id:Uuid,
    pub full_name:String,
    pub email:String,
    pub password_hash:String,
    pub role:UserRole,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>,
}