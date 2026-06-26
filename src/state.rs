use sqlx::PgPool;

use crate::repositories::user_repo::UserRepo;
#[derive(Clone)]
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
