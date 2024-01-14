use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::auth::check_admin::check_admin;
use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::utils::get_user::get_user;
use crate::AppState;

pub async fn admin_get_users(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    check_admin(&session, &state.db).await?;
    let users = sqlx::query_as::<_, UserModel>("SELECT * FROM users")
        .fetch_all(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(200)
        .body(serde_json::to_string(&users).unwrap())
        .unwrap())
}

pub async fn admin_get_user(
    State(state): State<AppState>,
    session: Session,
    Path(payload): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    check_admin(&session, &state.db).await?;
    let user = get_user(&state.db, payload.to_string()).await?;

    match user {
        None => Err(AppError::NotFound {
            error: "User not found".to_string(),
        }),
        Some(user) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&user).unwrap())
            .unwrap()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    username: String,
    email: String,
    role: String,
}

pub async fn admin_update_user(
    State(state): State<AppState>,
    session: Session,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    check_admin(&session, &state.db).await?;

    let found_user = get_user(&state.db, user_id.to_string()).await?;
    if found_user.is_none() {
        return Err(AppError::NotFound {
            error: "User not found".to_string(),
        });
    }

    sqlx::query("UPDATE users SET username = $1, email = $2, role = $3 WHERE id = $4")
        .bind(payload.username)
        .bind(payload.email)
        .bind(payload.role)
        .bind(user_id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::UPDATED)
}

pub async fn admin_delete_user(
    State(state): State<AppState>,
    session: Session,
    Path(payload): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_admin(&session, &state.db).await?;

    if payload == user.id {
        return Err(AppError::NotAllowed {
            error: "Cannot delete yourself".to_string(),
        });
    }

    let user = get_user(&state.db, payload.to_string()).await?;

    if user.is_none() {
        return Err(AppError::NotFound {
            error: "User not found".to_string(),
        });
    }

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(payload)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}
