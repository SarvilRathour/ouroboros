use axum::Json;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::env;
mod handlers;
mod models;
mod repositories;
mod schemas;
mod state;
mod auth;
use handlers::health::health_check;
use handlers::user::register;
use models::user::User;
use repositories::user_repo;
use schemas::user_schema::RegisterUserRequest;
use state::AppState;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let app_state = AppState::new_database(&database_url)
        .await
        .expect("failed to connect to database");
    println!("Database connected succesfully");
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/seed/users", post(register))
        
        // .route("/task", post(task_create))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
