use crate::repositories::user_repo::UserRepo;
use axum::extract::FromRef;
use sqlx::PgPool;
#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub user_repo: UserRepo,
}
impl AppState {
    pub async fn new_database(url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(url).await?;
        let user_repo = UserRepo::new(db.clone());
        Ok(Self { db, user_repo })
    }
}
