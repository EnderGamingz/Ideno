use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::certification::CertificationModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

pub async fn get_certifications(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let certifications =
        sqlx::query_as::<_, CertificationModel>("SELECT * FROM certification WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&certifications).unwrap())
        .unwrap())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddCertificationPayload {
    name: String,
    organization: String,
    issue_date: Option<i64>,
    expiration_date: Option<i64>,
    credential_id: Option<String>,
    credential_url: Option<String>,
}

pub async fn add_certification(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddCertificationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let count =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM certification WHERE user_id = $1")
            .bind(user.id)
            .fetch_one(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    if count.0 >= 50 {
        return Err(AppError::DataConflict {
            error: "Certification limit reached".to_string(),
        });
    }

    let new_certification = sqlx::query_as::<_, (i32, )>
        ("INSERT INTO certification (user_id, name, organization, issue_date, expiration_date, credential_id, credential_url) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
        .bind(user.id)
        .bind(payload.name)
        .bind(payload.organization)
        .bind(payload.issue_date)
        .bind(payload.expiration_date)
        .bind(payload.credential_id)
        .bind(payload.credential_url)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::CREATED {
        id: Some(new_certification.0),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateCertificationPayload {
    id: i32,
    name: String,
    organization: String,
    issue_date: Option<i64>,
    expiration_date: Option<i64>,
    credential_id: Option<String>,
    credential_url: Option<String>,
}

pub async fn update_certification(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<UpdateCertificationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    let entry_does_exist = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM certification WHERE id = $1 AND user_id = $2",
    )
    .bind(payload.id)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)
    .map_or_else(|_| None, |count| Some(count.0 > 0));

    if !entry_does_exist.unwrap() {
        return Err(AppError::NotFound {
            error: "Certification not found".to_string(),
        });
    }

    sqlx::query("UPDATE certification SET name = $1, organization = $2, issue_date = $3, expiration_date = $4, credential_id = $5, credential_url = $6 WHERE id = $7 AND user_id = $8")
        .bind(payload.name)
        .bind(payload.organization)
        .bind(payload.issue_date)
        .bind(payload.expiration_date)
        .bind(payload.credential_id)
        .bind(payload.credential_url)
        .bind(payload.id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::UPDATED)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteCertificationPayload {
    id: i32,
}

pub async fn delete_certification(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<DeleteCertificationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &*state.db).await?;

    sqlx::query("DELETE FROM certification WHERE id = $1 AND user_id = $2")
        .bind(payload.id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}
