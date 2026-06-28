use crate::models::user::UserRole;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn generate_token(
    user_id: Uuid,
    secret: &str,
    role: &UserRole,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token| token.claims)
}
