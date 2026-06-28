use crate::models::user::CreateTaskRequest;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Json;
use serde_json::json;
use serde_json::Value;
use std::fmt::Display;
use crate::AppState;

pub async fn task_create(State(state):State<AppState>,Json(payload):Json<CreateTaskRequest>)->Json<Value>{
    
    Json(json!({ "message": "Task created successfully" }))
}