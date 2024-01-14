use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use sqlx::{Pool, Sqlite};

pub async fn get_user(db: &Pool<Sqlite>, user_id: String) -> Result<Option<UserModel>, AppError> {
    let user_result = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
        .bind(user_id)
        .fetch_optional(db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch user: {}", err);
            AppError::InternalError
        })?;

    Ok(user_result)
}
