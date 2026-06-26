use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
};
use uuid::Uuid;
use crate::{auth::jwt::validate_token, models::user::UserRole, state::AppState};
pub struct auth_user {
    pub user_id: String,
    pub username: String,
    pub role: UserRole,
}
impl<S> FromRequestParts<S> for auth_user
where
    AppState: FromRef<S>,
    S: Send+Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let headers = &parts.headers;
        let token = extract_token(headers);
        if let Some(token) = token {
            let jwt_secret = std::env::var("SECRET").unwrap();
            let claims = validate_token(&token, &jwt_secret)
                .map_err(|e| {
                    eprintln!("JWT VALIDATION FAILED: {:?}", e);
                    StatusCode::UNAUTHORIZED
                })?;
            println!("SUB: {}", claims.sub);
            let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
            let user = app_state.user_repo.find_by_id(&user_id)
                .await
                .map_err(|e| {
                    eprintln!("DB LOOKUP FAILED: {:?}", e);
                    StatusCode::UNAUTHORIZED
                })?;
            let auth_user = auth_user {
                user_id: user_id.to_string(),
                username: user.full_name,
                role: user.role,
            };
            Ok(auth_user)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
fn extract_token(headers: &axum::http::HeaderMap) -> Option<&str> {
    let auth_header = headers.get("Authorization")?;
    let token = auth_header.to_str().ok()?;
    println!("TOKEN: {:?}", token);
    if token.starts_with("Bearer ") {
        Some(&token[7..])
    } else {
        None
    }
}
