use sqlx::{Pool, Sqlite};
use tower_sessions::Session;

use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::utils::get_user;

pub async fn check_admin(session: &Session, db: &Pool<Sqlite>) -> Result<UserModel, AppError> {
    let user_id = match session.get::<String>("user_id").await.unwrap() {
        Some(user) => user,
        None => return Err(AppError::NotLoggedIn)?,
    };

    let user_result = get_user::get_user(db, user_id).await?;

    if let Some(user) = user_result {
        if user.role == "admin" {
            Ok(user)
        } else {
            return Err(AppError::Forbidden { error: None })?;
        }
    } else {
        return Err(AppError::Forbidden { error: None })?;
    }
}
