use axum::Json;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::env;
mod auth;
mod handlers;
mod models;
mod repositories;
mod schemas;
mod state;
use crate::handlers::two_fa_login::two_factor_login;
use auth::jwt;
use handlers::auth_login::login;
use handlers::curr_user::get_current_user;
use handlers::health::health_check;
use handlers::task::task_create;
use handlers::user::register;
use handlers::view_task::task_view_my_tasks;
use models::user::User;
use repositories::user_repo;
use schemas::user_schema::RegisterUserRequest;
use state::AppState;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be in .env file or environment");
    env::var("REDIS_URL").expect("REDIS_URL must be set in .env file or environment");
    let app_state = AppState::new_database(&database_url, &redis_url)
        .await
        .expect("failed to connect to database");
    println!("Database connected succesfully");
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/seed/users", post(register))
        .route("/current_user", get(get_current_user))
        .route("/auth/login", post(login))
        .route("/auth/2fa_login", post(two_factor_login))
        .route("/task", post(task_create))
        .route("/task/view-my-tasks", get(task_view_my_tasks))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
