use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::models::user::{PublicAuthUserModel, UserModel};
use crate::response::error_handling::AppError;
use crate::services::session_service::SessionService;
use crate::{IdenoDBResult, IdenoPool};

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone)]
pub struct UserService {
    db_pool: IdenoPool,
}

impl UserService {
    pub fn new(db_pool: IdenoPool) -> Self {
        UserService { db_pool }
    }

    pub async fn check_admin(&self, session: &Session) -> Result<UserModel, AppError> {
        let user = self.check_user(&session).await?;

        match user.role.as_str() {
            "admin" => Ok(user),
            _ => Err(AppError::Forbidden { error: None }),
        }
    }

    pub async fn check_user(&self, session: &Session) -> Result<UserModel, AppError> {
        let user_id = SessionService::check_logged_in(session).await?;
        let user_result = self.get_user(user_id).await?;

        match user_result {
            Some(user) => Ok(user),
            None => Err(AppError::UserNotFound)?,
        }
    }

    pub async fn get_user_by_email_or_username(
        &self,
        email: &String,
        username: &String,
    ) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(username)
            .bind(email)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn get_user_by_either_email_or_username(
        &self,
        email_or_username: String,
    ) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(email_or_username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn get_public_auth_user(
        &self,
        user_id: String,
    ) -> Result<PublicAuthUserModel, AppError> {
        sqlx::query_as::<_, PublicAuthUserModel>(
            "SELECT id, username, email, created_at FROM users WHERE id = ?",
        )
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::UserNotFound)
    }

    pub async fn check_user_optional(
        &self,
        session: &Session,
    ) -> Result<Option<UserModel>, AppError> {
        let user_id = SessionService::get_session_id(session).await;

        match user_id {
            Some(user) => self.get_user(user).await,
            None => Ok(None),
        }
    }

    pub async fn get_user(&self, user_id: String) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<UserModel, AppError> {
        let user = sqlx::query_as::<_, UserModel>("SELECT * FROM users where username = ?")
            .bind(username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)?;
        match user {
            Some(user) => Ok(user),
            None => Err(AppError::UserNotFound)?,
        }
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn update_user(
        &self,
        user_id: i32,
        payload: UpdateUserRequest,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET username = $1, email = $2, role = $3 WHERE id = $4")
            .bind(payload.username)
            .bind(payload.email)
            .bind(payload.role)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn admin_get_user(&self, user_id: i32) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|err| {
                tracing::error!("Failed to fetch user: {}", err);
                AppError::InternalError
            })
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }
}
