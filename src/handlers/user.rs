use crate::auth::{
    jwt::{generate_token, validate_token},
    password::hash_password,
};
use crate::state::AppState;
use crate::{auth::password, schemas::user_schema::RegisterUserData};
use axum::{Json, extract::State};
use serde_json::{Value, json};
use std::env;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserData>,
) -> Json<Value> {
    let password_hash = hash_password(&payload.password).unwrap();
    let user = state
        .user_repo
        .create(
            &payload.full_name,
            &payload.email,
            &password_hash,
            &payload.role,
        )
        .await
        .unwrap();
    let secret = env::var("SECRET").expect("SECRET must be set in .env file or environment");
    let token = generate_token(user.id, &secret, &payload.role).unwrap();

    Json(json!({
        "message":"received",
        "full_name":&payload.full_name,
        "token":token
    }))
}
