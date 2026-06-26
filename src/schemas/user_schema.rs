use serde::{Serialize,Deserialize};
use sqlx::prelude::FromRow;
use crate::models::user::UserRole;
#[derive(Debug,Deserialize)]
pub struct RegisterUserRequest{
    pub user:RegisterUserData,
}
#[derive(Debug,Deserialize,FromRow)]
pub struct RegisterUserData{
    pub full_name:String,
    pub email:String,
    pub password:String,
    pub role:UserRole,
}