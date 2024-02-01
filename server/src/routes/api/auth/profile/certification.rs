use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::certification_service::{AddCertificationPayload, UpdateCertificationPayload};
use crate::AppState;

pub async fn get_certifications(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let certifications = state
        .certification_service
        .get_all_certifications(user.id)
        .await?;

    Ok(Json(serde_json::to_value(&certifications).unwrap()))
}

pub async fn add_certification(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddCertificationPayload>,
) -> Result<AppSuccess, AppError> {
    let user = state.user_service.check_user(&session).await?;

    let count = state
        .certification_service
        .get_certification_count(user.id)
        .await?;

    if count >= 50 {
        return Err(AppError::DataConflict {
            error: "Certification limit reached".to_string(),
        });
    }

    let new_certification = state
        .certification_service
        .create_certification(user.id, payload)
        .await?;

    Ok(AppSuccess::CREATED {
        id: Some(new_certification),
    })
}

pub async fn update_certification(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCertificationPayload>,
) -> Result<AppSuccess, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .certification_service
        .user_owns_certification(user.id, id)
        .await?;

    state
        .certification_service
        .update_certification(user.id, id, payload)
        .await?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_certification(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.check_user(&session).await?;

    state
        .certification_service
        .user_owns_certification(user.id, id)
        .await?;

    state
        .certification_service
        .delete_certification(user.id, id)
        .await?;

    Ok(AppSuccess::DELETED)
}
