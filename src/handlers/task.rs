use crate::AppState;
use crate::auth::middleware::Claim;
use crate::models::user::CreateTaskRequest;
use axum::extract::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use serde_json::Value;
use serde_json::json;
use std::fmt::Display;
use uuid::Uuid;
// #[derive(Debug, Deserialize, FromRow)]
// pub struct CreateTaskRequest {
//     pub title: String,
//     pub description:String,
//     pub status:task_status,
//     pub priority:task_priority,
//     pub assign_to_email:String,
// }
// pub async fn create_task(&self,title:&str,description:&str,status:&task_status,priority:&task_priority,created_by:&Uuid,assign_to:&Uuid)

pub async fn task_create(
    State(state): State<AppState>,
    claims: Claim,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Value>, StatusCode> {
    if claims.role != "Admin" {
        println!("User is not an admin, role: {}", claims.role);
        return Err(StatusCode::FORBIDDEN);
    }
    let assigned_to = match state
        .user_repo
        .find_by_email(&payload.assign_to_email)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            println!("Error occured finding email: {}", e);
            return Err(StatusCode::NOT_FOUND);
        }
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(uuid) => uuid,
        Err(e) => {
            println!("Error occured parsing user_id: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let task = match state
        .user_repo
        .create_task(
            &payload.title,
            &payload.description,
            &payload.status,
            &payload.priority,
            &user_id,
            &assigned_to.id,
        )
        .await
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create task in DB: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(json!({ "message": "Task created successfully"})))
}
