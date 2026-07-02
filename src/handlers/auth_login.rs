use crate::AppState;
use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::handlers::two_fa_login::two_factor_login;
use crate::schemas::user_schema::LoginUserRequest;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use rand::Rng;
use serde_json::{Value, json};
use std::env;
use std::fmt;
#[derive(Debug)]
pub enum EmailError {
    SendFailed,
    InvalidEmail,
}
impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::SendFailed => write!(f, "SendFailed"),
            EmailError::InvalidEmail => write!(f, "InvalidEmail"),
        }
    }
}
impl std::error::Error for EmailError {}
fn generate_6_digit_code() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..=999999);
    format!("{:06}", number)
}
pub async fn send_verification_email(email: &str, code: &str) -> Result<(), EmailError> {
    let smtp_username = env::var("SMTP_EMAIL").unwrap_or_default();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verification Code")
        .body(format!("Your verification code is: {}", code))
        .unwrap();
    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();
    mailer
        .send(email)
        .await
        .map_err(|_| EmailError::SendFailed)?;
    Ok(())
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

    if let Err(e) = send_verification_email(&user.email, &code).await {
        eprintln!("Failed to send email: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(json!({
        "message": "Challenge created successfully and verification email sent",
        "challenge": challenge.id,
        "expires_at": challenge.expires_at,
    })))

    // let secret = env::var("SECRET").expect("SECRET must be set in .env file or environment");
    // let token = generate_token(user.id, &secret, &user.role).unwrap();
}
