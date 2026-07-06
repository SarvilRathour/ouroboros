use crate::repositories::user_repo::UserRepo;
use axum::extract::FromRef;
use redis::Client;
use sqlx::PgPool;
#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub user_repo: UserRepo,
    pub redis_client: Client,
}
impl AppState {
    pub async fn new_database(url: &str, redis_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(url).await?;
        let user_repo = UserRepo::new(db.clone());
        let redis_client = Client::open(redis_url).unwrap();
        Ok(Self {
            db,
            user_repo,
            redis_client,
        })
    }
}
