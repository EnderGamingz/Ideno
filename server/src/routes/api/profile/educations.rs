use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::{Pool, Sqlite};
use tower_sessions::Session;

use crate::AppState;
use crate::auth::check_user::{check_user_by_username, optional_user};
use crate::models::education::{AuthEducationModel, PublicEducationModel};
use crate::response::error_handling::AppError;

pub async fn get_public_educations(
    State(state): State<AppState>,
    session: Session,
    Path(identifier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let logged_in_user = optional_user(&session, &state.db).await?;
    let user = check_user_by_username(identifier, &state.db).await?;

    return if let Some(logged_in_user) = logged_in_user {
        let educations = auth_educations(logged_in_user.id, &state.db).await?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&educations).unwrap())
            .unwrap())
    } else {
        let educations = public_educations(user.id, &state.db).await?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&educations).unwrap())
            .unwrap())
    };
}

async fn public_educations(
    user_id: i32,
    db: &Arc<Pool<Sqlite>>,
) -> Result<Vec<PublicEducationModel>, AppError> {
    sqlx::query_as::<_, PublicEducationModel>(
        "SELECT
                school,
                degree,
                field,
                start_date,
                end_date
              FROM educations
              WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_all(&**db)
    .await
    .map_err(|_| AppError::InternalError)
}

async fn auth_educations(
    user_id: i32,
    db: &Arc<Pool<Sqlite>>,
) -> Result<Vec<AuthEducationModel>, AppError> {
    sqlx::query_as::<_, AuthEducationModel>(
        "SELECT
                id,
                school,
                degree,
                field,
                start_date,
                end_date
              FROM educations
              WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_all(&**db)
    .await
    .map_err(|_| AppError::InternalError)
}
