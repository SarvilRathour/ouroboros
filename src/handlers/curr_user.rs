use crate::auth::middleware::auth_user;
use axum::extract::State;
use crate::state::AppState;
use axum::response::Json;
use serde_json::json;
// let auth_user = auth_user {
//     user_id: user_id.to_string(),
//     username: user.full_name,
//     role: user.role,
// };
pub async fn get_current_user(State(state): State<AppState>, auth: auth_user) -> Json<serde_json::Value> {
    Json(json!({
        "user_id":auth.user_id,
        "username":auth.username,
        "role":auth.role
    }))
}
