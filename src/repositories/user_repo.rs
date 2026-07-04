use chrono::DateTime;
use chrono::Utc;
use sqlx::{PgPool, Postgres};

use uuid::Uuid;
#[derive(Clone)]
pub struct UserRepo {
    pub db: PgPool,
}
use crate::models::user::{LoginChallenge, User, UserRole,Task};
use crate::models::user::{task_priority, task_status};
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
    pub async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE Email=$1")
            .bind(email)
            .fetch_one(&self.db)
            .await?;
        Ok(user)
    }
    pub async fn create_challenge(
        &self,
        user_id: &Uuid,
        code: &str,
        expires_at: &DateTime<Utc>,
    ) -> Result<Uuid, sqlx::Error> {
        let record_id = sqlx::query_scalar::<Postgres, Uuid>(
            "INSERT INTO login_challenges(user_id,code,expires_at) VALUES($1,$2,$3) RETURNING id",
        )
        .bind(user_id)
        .bind(code)
        .bind(expires_at)
        .fetch_one(&self.db)
        .await?;
        Ok(record_id)
    }
    pub async fn find_challenge_by_id(&self, id: &Uuid) -> Result<LoginChallenge, sqlx::Error> {
        let challenge = sqlx::query_as::<Postgres, LoginChallenge>(
            "SELECT * FROM login_challenges WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(challenge)
    }
    pub async fn delete_challenge(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM login_challenges WHERE id=$1")
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn create_task(
        &self,
        title: &str,
        description: &str,
        status: &task_status,
        priority: &task_priority,
        created_by: &Uuid,
        assign_to: &Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO tasks (title,description,status,priority,created_by_id,assigned_to_id) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(title)
            .bind(description)
            .bind(status)
            .bind(priority)
            .bind(created_by)
            .bind(assign_to)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn find_tasks_by_user(&self, user_id: &Uuid) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as::<Postgres, Task>(
            "SELECT * FROM tasks WHERE assigned_to_id = $1",
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;
        Ok(tasks)
    }
}
