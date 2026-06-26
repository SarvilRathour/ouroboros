use axum::{extract::State,Json};
use crate::state::AppState;
use serde_json::{json,Value};
pub async fn health_check(State(state):State<AppState>)->Json<Value>{
    match sqlx::query("SELECT 1").execute(&state.db).await{
     Ok(_)=>Json(json!({
         "status":"ok",
         "database":"connected"
     })),
     Err(e)=>Json(json!({
         "status":"error",
         "database":"disconnected",
         "error":e.to_string()
     }))
    }
}