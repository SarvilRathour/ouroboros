use serde::{Serialize,Deserialize};
use crate::models::user::UserRole;
#[derive(Debug,Deserialize)]
pub struct RegisterUserRequest{
    pub user:RegisterUserData,
}
#[derive(Debug,Deserialize)]
pub struct RegisterUserData{
    pub full_name:String,
    pub email:String,
    pub password:String,
    pub role:UserRole,
}