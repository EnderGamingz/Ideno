use sqlx::{Pool, Sqlite};
use tower_sessions::Session;

use crate::AppError;

use crate::models::user::UserModel;

pub async fn check_user(session: &Session, db: &Pool<Sqlite>) -> Result<UserModel, AppError> {
    let user_id = match session.get::<String>("user_id").await.unwrap() {
        Some(user) => user,
        None => return Err(AppError::NotLoggedIn)?,
    };

    let user_result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch user: {}", err);
            AppError::InternalError
        })?;

    match user_result {
        Some(user) => Ok(user),
        None => Err(AppError::UserNotFound)?,
    }
}
