use axum::{Router, extract::State, routing::{get,post}};
use std::env;
use serde_json::{json,Value};
use axum::Json;
mod state;
mod handlers;
mod models;
mod schemas;
mod repositories;
use schemas::user_schema::RegisterUserRequest;
use state::AppState;
use handlers::health::health_check;
use handlers::user::register;
use repositories::user_repo;
use models::user::User;
#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let app_state=AppState::new_database(&database_url).await.expect("failed to connect to database");
    println!("Database connected succesfully");
    let app=Router::new()
        .route("/health",get(health_check))
        .route("/seed/users",post(register))
        .with_state(app_state);
    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}