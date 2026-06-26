use crate::AppState;
use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::schemas::user_schema::LoginUserRequest;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rand::Rng;
use serde_json::{Value, json};
use std::env;
fn generate_6_digit_code() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..=999999);
    format!("{:06}", number)
}
pub async fn login(
    State(appstate): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    let user = appstate.user_repo.find_by_email(&payload.email).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    let password_valid = verify_password(&payload.password, &user.password_hash).unwrap();
    if !password_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let code = generate_6_digit_code();
    let expire_at = chrono::Utc::now() + chrono::Duration::minutes(15);
    let challenge = appstate
        .user_repo
        .create_challenge(&user.id, &code, &expire_at)
        .await;
    let challenge = match challenge {
        Ok(challenge) => challenge,
        Err(e) => {
            eprintln!("Error fetching challenge id from DB: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let challenge = appstate.user_repo.find_challenge_by_id(&challenge).await;
    let challenge = match challenge {
        Ok(challenge) => challenge,
        Err(e) => {
            eprintln!("Error fetching challenge challenge from DB: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(json!({
        "message": "Challenge created successfully",
        "challenge": challenge.id,
        "code":challenge.code,
        "expires_at": challenge.expires_at,
    })))

    // let secret = env::var("SECRET").expect("SECRET must be set in .env file or environment");
    // let token = generate_token(user.id, &secret, &user.role).unwrap();
}
