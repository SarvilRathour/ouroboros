use axum::{Json};
use serde_json::{json,Value};

use crate::schemas::user_schema::RegisterUserData;
pub async fn register(Json(payload):Json<RegisterUserData>)->Json<Value>{
    println!("{}",payload.full_name);
    println!("{}",payload.email);
    println!("{}",payload.password);
    println!("{:#?}",payload.role);
    Json(json!({
        "message":"received",
        "full_name":&payload.full_name
    }))
}