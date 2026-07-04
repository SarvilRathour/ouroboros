use crate::auth::jwt::generate_token;
use crate::{AppState, models::user::TwoFactorLogin};
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;
use std::env;
use crate::auth::middleware::Claim;
use crate::models::user::Task;
pub async fn task_view_my_tasks(claims:Claim,State(appstate):State<AppState>) -> Result<Json<Value>, StatusCode> {
    if claims.role!="Staff"{
        return Err(StatusCode::UNAUTHORIZED);
    }
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(uuid) => uuid,
        Err(e) => {
            println!("Error occured parsing user_id: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let task = match appstate.user_repo.find_tasks_by_user(&user_id).await {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Error occured finding tasks: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(json!(task)))
}