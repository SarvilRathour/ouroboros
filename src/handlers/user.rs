use crate::schemas::user_schema::RegisterUserData;
use crate::state::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserData>,
) -> Json<Value> {
    let user = state
        .user_repo
        .create(
            &payload.full_name,
            &payload.email,
            &payload.password,
            &payload.role,
        )
        .await
        .unwrap();
    println!("{:?}", user);
    Json(json!({
        "message":"received",
        "full_name":&payload.full_name
    }))
}
