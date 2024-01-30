use crate::utils::get_user;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tower_sessions::Session;

use crate::response::error_handling::AppError;

use crate::models::user::UserModel;

pub async fn check_user(session: &Session, db: &Pool<Sqlite>) -> Result<UserModel, AppError> {
    let user_id = match session.get::<String>("user_id").await.unwrap() {
        Some(user) => user,
        None => return Err(AppError::NotLoggedIn)?,
    };

    let user_result = get_user::get_user(db, user_id).await?;

    match user_result {
        Some(user) => Ok(user),
        None => Err(AppError::UserNotFound)?,
    }
}

pub async fn optional_user(
    session: &Session,
    db: &Pool<Sqlite>,
) -> Result<Option<UserModel>, AppError> {
    let user_id = session.get::<String>("user_id").await.unwrap();

    match user_id {
        Some(user) => {
            get_user::get_user(db, user).await
        },
        None => Ok(None),
    }
}

pub async fn check_user_by_username(
    username: String,
    db: &Arc<Pool<Sqlite>>,
) -> Result<UserModel, AppError> {
    let user_id = sqlx::query_as::<_, UserModel>("SELECT * FROM users where username = ?")
        .bind(username)
        .fetch_optional(&**db)
        .await
        .map_err(|_| AppError::InternalError)?;

    if user_id.is_none() {
        return Err(AppError::NotFound {
            error: "Profile not Found".to_string(),
        })?;
    }

    return Ok(user_id.unwrap());
}
