use crate::utils::get_user;
use sqlx::{Pool, Sqlite};
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
