use axum::extract::{Path, State};
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::user_service::UpdateUserRequest;
use crate::AppState;

pub async fn admin_get_users(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    state.user_service.check_admin(&session).await?;
    let users = state.user_service.get_all_users().await?;

    Ok(Json(serde_json::to_value(users).unwrap()))
}

pub async fn admin_get_user(
    State(state): State<AppState>,
    session: Session,
    Path(payload): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.user_service.check_admin(&session).await?;
    let user = state.user_service.admin_get_user(payload).await?;

    match user {
        None => Err(AppError::NotFound {
            error: "User not found".to_string(),
        }),
        Some(user) => Ok(Json(serde_json::to_value(user).unwrap())),
    }
}

pub async fn admin_update_user(
    State(state): State<AppState>,
    session: Session,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<AppSuccess, AppError> {
    state.user_service.check_admin(&session).await?;
    let found_user = state.user_service.admin_get_user(user_id).await?;

    let user = match found_user {
        Some(user) => user,
        None => {
            return Err(AppError::NotFound {
                error: "User not found".to_string(),
            });
        }
    };

    state.user_service.update_user(user.id, payload).await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn admin_delete_user(
    State(state): State<AppState>,
    session: Session,
    Path(payload): Path<i32>,
) -> Result<AppSuccess, AppError> {
    let admin_check_result = state.user_service.check_admin(&session).await?;
    if payload == admin_check_result.id {
        return Err(AppError::NotAllowed {
            error: "Cannot delete yourself".to_string(),
        });
    }

    let user = state.user_service.admin_get_user(payload).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::NotFound {
                error: "User not found".to_string(),
            });
        }
    };

    state.user_service.delete_user(user.id).await?;

    Ok(AppSuccess::DELETED)
}
