use crate::auth::jwt::generate_token;
use crate::{AppState, models::user::TwoFactorLogin};
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};
use std::env;
pub async fn task_view_my_tasks(State(appstate):State<AppState>,Json(payload):Json<Value>) -> Result<Json<Value>, StatusCode> {
    
}