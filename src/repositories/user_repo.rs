use sqlx::{PgPool, Postgres};
use uuid::Uuid;
#[derive(Clone)]
pub struct UserRepo {
    pub db: PgPool,
}
use crate::models::user::{User, UserRole};

impl UserRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    pub async fn create(
        &self,
        full_name: &str,
        email: &str,
        password_hash: &str,
        role: &UserRole,
    ) -> Result<User, sqlx::Error> {
        let user: User = sqlx::query_as::<Postgres, User>(
            "INSERT INTO users(full_name,email,password_hash,role) VALUES($1,$2,$3,$4) RETURNING *",
        )
        .bind(full_name)
        .bind(email)
        .bind(password_hash)
        .bind(role)
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
    pub async fn find_by_id(&self, id: &Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(user)
    }
    pub async fn find_by_email(&self,email:&str)->Result<User,sqlx::Error>{
        let user=sqlx::query_as::<Postgres,User>("SELECT * FROM users WHERE Email=$1")
            .bind(email)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }
}
