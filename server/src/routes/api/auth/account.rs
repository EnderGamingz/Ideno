use crate::auth::check_user::check_user;
use crate::{AppError, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

#[derive(serde::Deserialize)]
pub struct AccountUpdatePayload {
    username: Option<String>,
    email: Option<String>,
}

pub async fn update_account(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AccountUpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    if payload.username.is_some() && payload.email.is_some() {
        return Err(AppError::BadRequest {
            error: Some("Cannot update both username and email at the same time".to_string()),
        })?;
    }

    if payload.username.is_none() && payload.email.is_none() {
        return Err(AppError::BadRequest {
            error: Some("Must update either username or email".to_string()),
        })?;
    }

    let new_username_exists = if let Some(ref username) = payload.username {
        sqlx::query("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| return AppError::InternalError)?
            .is_some()
    } else {
        false
    };

    let new_email_exists = if let Some(ref email) = payload.email {
        sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&*state.db)
            .await
            .map_err(|_| return AppError::InternalError)?
            .is_some()
    } else {
        false
    };

    let new_value_type = if payload.username.is_some() {
        "username"
    } else {
        "email"
    };

    if new_username_exists || new_email_exists {
        return Err(AppError::BadRequest {
            error: Some(format!("Value already in use: {}", &new_value_type)),
        })?;
    }

    let new_value = match new_value_type {
        "username" => payload.username.unwrap(),
        "email" => payload.email.unwrap(),
        _ => return Err(AppError::InternalError)?,
    };

    sqlx::query(format!("UPDATE users SET {} = $1 WHERE id = $2", new_value_type).as_str())
        .bind(new_value)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| return AppError::InternalError)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(serde::Deserialize)]
pub struct PasswordUpdatePayload {
    old_password: String,
    new_password: String,
}

pub async fn update_password(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<PasswordUpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let password_match = bcrypt::verify(&payload.old_password, &user.password)
        .map_err(|_| return AppError::InternalError)?;

    if !password_match {
        return Err(AppError::BadRequest {
            error: Some("Old password does not match".to_string()),
        })?;
    }

    let is_same_password = bcrypt::verify(&payload.new_password, &user.password)
        .map_err(|_| return AppError::InternalError)?;

    if is_same_password {
        return Err(AppError::BadRequest {
            error: Some("New password cannot match old password".to_string()),
        })?;
    }

    let hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST)
        .map_err(|_| return AppError::InternalError)?;

    sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(hash)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| return AppError::InternalError)?;

    Ok(StatusCode::OK.into_response())
}

pub async fn delete_account(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| return AppError::InternalError)?;

    Ok(StatusCode::OK.into_response())
}
