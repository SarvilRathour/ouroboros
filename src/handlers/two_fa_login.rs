use crate::auth::jwt::generate_token;
use crate::{AppState, models::user::TwoFactorLogin};
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};
use std::env;
// #[derive(Deserialize)]
// struct TwoFactorLoginPayload {
//     code:String,

// }

pub async fn two_factor_login(
    State(appstate): State<AppState>,
    Json(payload): Json<TwoFactorLogin>,
) -> Result<Json<Value>, StatusCode> {
    let challenge = appstate.user_repo.find_challenge_by_id(&payload.id).await;
    let challenge = match challenge {
        Ok(challenge) => challenge,
        Err(e) => {
            eprintln!("Error fetching challenge challenge from DB: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    if challenge.expires_at < Utc::now() {
        eprintln!("Expired token");
        return Err(StatusCode::UNAUTHORIZED);
    }
    if challenge.code != payload.code {
        eprintln!("Invalid code");
        return Err(StatusCode::UNAUTHORIZED);
    }
    let user = appstate.user_repo.find_by_id(&challenge.user_id).await;
    let user = match user {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error fetching user from DB: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let secret = env::var("SECRET").expect("SECRET must be set in .env file or environment");
    let token = generate_token(user.id, &secret, &user.role).unwrap();
    Ok(Json(json!({
        "message": "Login successful",
        "token": token
    })))
}
