use uuid::Uuid;
use chrono::{DateTime,Utc};
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize,Debug)]
// #[serde(rename_all="PascalCase")]
pub enum UserRole{
    Admin,
    Staff
}
#[derive(Serialize, Deserialize,Debug)]
pub struct User{
    pub id:Uuid,
    pub full_name:String,
    pub email:String,
    pub password_hash:String,
    pub role:UserRole,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>
}