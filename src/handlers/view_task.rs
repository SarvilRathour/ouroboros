use crate::auth::jwt::generate_token;
use crate::auth::middleware::Claim;
use crate::models::user::Task;
use crate::{AppState, models::user::TwoFactorLogin};
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{Value, json};
use std::env;
use uuid::Uuid;
pub async fn task_view_my_tasks(
    claims: Claim,
    State(appstate): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    if claims.role != "Staff" {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let user_id_str = &claims.sub;
    let cache_key = format!("user_tasks:{}", user_id_str);
    let mut redis_conn = match appstate
        .redis_client
        .get_multiplexed_async_connection()
        .await
    {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Redis connection failed:{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let cached_tasks: redis::RedisResult<String> = redis_conn.get(&cache_key).await;
    let cached_tasks = match cached_tasks {
        Ok(tasks) => {
            println!("Cache HIT for user:{}", user_id_str);
            let tasks: Value = serde_json::from_str(&tasks).unwrap();
            return Ok(Json(json!({
                "cache_hit":true,
                "tasks":tasks
            })))
        }
        Err(e) => {
            println!("Redis get failed: {}", e);
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
            
        }
    };
 
    Ok(Json(json!(task)))

}
