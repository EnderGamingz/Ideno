use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::contact_information::ContactInformationModel;
use crate::{AppError, AppState};

pub async fn get_contact_information(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let contact_information = sqlx::query_as::<_, ContactInformationModel>(
        "SELECT * FROM contact_information WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&contact_information).unwrap())
        .unwrap())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddContactInformationPayload {
    contact_type: String,
    value: String,
}

pub async fn add_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let count =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE user_id = $1")
            .bind(user.id)
            .bind(&payload.contact_type)
            .bind(&payload.value)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    if count.0 >= 50 {
        return Err(AppError::DataConflict {
            error: "Contact information limit reached".to_string(),
        });
    }

    let contact_info_result = sqlx::query_as::<_, ContactInformationModel>(
        "SELECT * FROM contact_information WHERE user_id = $1 AND type_field = $2 AND value = $3 LIMIT 1",
    )
    .bind(user.id)
    .bind(&payload.contact_type)
    .bind(&payload.value)
    .fetch_all(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)?;

    if !contact_info_result.is_empty() {
        return Err(AppError::DataConflict {
            error: "Contact information already exists".to_string(),
        });
    }

    sqlx::query("INSERT INTO contact_information (user_id, type_field, value) VALUES ($1, $2, $3)")
        .bind(user.id)
        .bind(payload.contact_type)
        .bind(payload.value)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteContactInformationPayload {
    id: i32,
}

pub async fn delete_contact_information(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<DeleteContactInformationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    sqlx::query("DELETE FROM contact_information WHERE id = $1 AND user_id = $2")
        .bind(payload.id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(StatusCode::OK.into_response())
}
