use axum::http::StatusCode;
use crate::schemas::user_schema::LoginUserRequest;
use axum::response::IntoResponse;
use crate::auth::password::verify_password;
use axum::extract::State;
use crate::AppState;
use axum::extract::Json;
use crate::auth::jwt::generate_token;
use std::env;

use serde_json::{Value, json};
pub async fn login(State(appstate): State<AppState>,Json(payload):Json<LoginUserRequest>) -> Result<Json<Value>,StatusCode> {
    let user=appstate.user_repo.find_by_email(&payload.email).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    let password_valid=verify_password(&payload.password, &user.password_hash).unwrap();
    if !password_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let secret = env::var("SECRET").expect("SECRET must be set in .env file or environment");
    let token = generate_token(user.id, &secret, &user.role).unwrap();
    Ok(Json(json!({ "user": &user.email, "token": token, "role": &user.role })))
}
