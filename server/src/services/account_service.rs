use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Deserialize)]
pub struct AccountUpdatePayload {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PasswordUpdatePayload {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterCredentials {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AccountService {
    db_pool: IdenoPool,
}

impl AccountService {
    pub fn new(db_pool: IdenoPool) -> Self {
        AccountService { db_pool }
    }

    pub async fn create_account(
        &self,
        payload: RegisterCredentials,
        hash: String,
    ) -> Result<UserModel, AppError> {
        sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (email, username, password) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(&payload.email)
        .bind(&payload.username)
        .bind(hash)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| return AppError::InternalError)
    }

    pub async fn update_password(
        &self,
        user_id: i32,
        hash: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(hash)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    pub async fn update_username(
        &self,
        user_id: i32,
        username: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET username = $1 WHERE id = $2")
            .bind(username)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    pub async fn update_email(
        &self,
        user_id: i32,
        email: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
            .bind(email)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    pub async fn email_exists(&self, email: &String) -> bool {
        let result = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError);

        match result {
            Ok(Some(_row)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    pub async fn username_exists(&self, username: &String) -> bool {
        let result = sqlx::query("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError);

        match result {
            Ok(Some(_row)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }
}
