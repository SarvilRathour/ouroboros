use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState{
    pub db:PgPool,
}
impl AppState{
    pub async fn new_database(url:&str)->Result<Self,sqlx::Error>{
        let db=PgPool::connect(url).await?;
        Ok(Self{db})
    }
}