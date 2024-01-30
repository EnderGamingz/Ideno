use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::{Pool, Sqlite};
use tower_sessions::Session;

use crate::auth::check_user::{check_user_by_username, optional_user};
use crate::models::certification::{AuthCertificationModel, PublicCertificationModel};
use crate::response::error_handling::AppError;
use crate::AppState;

pub async fn get_public_certifications(
    State(state): State<AppState>,
    session: Session,
    Path(identifier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let logged_in_user = optional_user(&session, &state.db).await?;
    let user = check_user_by_username(identifier, &state.db).await?;

    return if let Some(logged_in_user) = logged_in_user {
        let certifications = auth_certifications(logged_in_user.id, &state.db).await?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&certifications).unwrap())
            .unwrap())
    } else {
        let certifications = public_certifications(user.id, &state.db).await?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&certifications).unwrap())
            .unwrap())
    };
}

async fn public_certifications(
    user_id: i32,
    db: &Arc<Pool<Sqlite>>,
) -> Result<Vec<PublicCertificationModel>, AppError> {
    sqlx::query_as::<_, PublicCertificationModel>(
        "SELECT
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_all(&**db)
    .await
    .map_err(|_| AppError::InternalError)
}

async fn auth_certifications(
    user_id: i32,
    db: &Arc<Pool<Sqlite>>,
) -> Result<Vec<AuthCertificationModel>, AppError> {
    sqlx::query_as::<_, AuthCertificationModel>(
        "SELECT
                id,
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_all(&**db)
    .await
    .map_err(|_| AppError::InternalError)
}
