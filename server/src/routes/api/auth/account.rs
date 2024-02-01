use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::account_service::{AccountUpdatePayload, PasswordUpdatePayload};
use crate::services::session_service::SessionService;
use crate::AppState;

pub async fn update_account(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AccountUpdatePayload>,
) -> Result<AppSuccess, AppError> {
    let user = state.user_service.check_user(&session).await?;

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

    let new_username_exists = state
        .account_service
        .username_exists(payload.username.clone().unwrap())
        .await;

    let new_email_exists = state
        .account_service
        .email_exists(payload.email.clone().unwrap())
        .await;

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

    match new_value_type {
        "username" => {
            state
                .account_service
                .update_username(user.id, payload.username.unwrap())
                .await?
        }
        "email" => {
            state
                .account_service
                .update_email(user.id, payload.email.unwrap())
                .await?
        }
        _ => return Err(AppError::InternalError)?,
    };

    Ok(AppSuccess::UPDATED)
}

pub async fn update_password(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<PasswordUpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

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

    state.account_service.update_password(user.id, hash).await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_account(
    State(state): State<AppState>,
    session: Session,
) -> Result<AppSuccess, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state.user_service.delete_user(user.id).await?;

    SessionService::flush_session(&session).await;

    Ok(AppSuccess::DELETED)
}
