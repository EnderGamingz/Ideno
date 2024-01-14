use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use tower_sessions::Session;

use crate::auth::check_user::check_user;
use crate::models::education::EducationModel;
use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

pub async fn get_educations(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &state.db).await?;

    let educations =
        sqlx::query_as::<_, EducationModel>("SELECT * FROM educations WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(&*state.db)
            .await
            .map_err(|_| AppError::InternalError)?;

    Ok(Response::builder()
        .status(axum::http::StatusCode::OK)
        .body(serde_json::to_string(&educations).unwrap())
        .unwrap())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddEducationPayload {
    school: String,
    degree: Option<String>,
    field: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

pub async fn add_education(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<AddEducationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &state.db).await?;

    let count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM educations WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    if count.0 >= 50 {
        return Err(AppError::DataConflict {
            error: "Education limit reached".to_string(),
        });
    }

    let created_education = sqlx::query_as::<_, (i64,)>("INSERT INTO educations (user_id, school, degree, field, start_date, end_date) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(user.id)
        .bind(payload.school)
        .bind(payload.degree)
        .bind(payload.field)
        .bind(payload.start_date)
        .bind(payload.end_date)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::CREATED {
        id: Some(created_education.0),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateEducationPayload {
    school: String,
    degree: Option<String>,
    field: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

pub async fn update_education(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEducationPayload>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &state.db).await?;

    education_exists_for_user_id(&state, id, &user).await?;

    sqlx::query("UPDATE educations SET school = $1, degree = $2, field = $3, start_date = $4, end_date = $5 WHERE id = $6 AND user_id = $7")
        .bind(payload.school)
        .bind(payload.degree)
        .bind(payload.field)
        .bind(payload.start_date)
        .bind(payload.end_date)
        .bind(id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::UPDATED)
}

pub async fn delete_education(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = check_user(&session, &state.db).await?;

    education_exists_for_user_id(&state, id, &user).await?;

    sqlx::query("DELETE FROM educations WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}

async fn education_exists_for_user_id(
    state: &AppState,
    payload: i32,
    user: &UserModel,
) -> Result<Option<bool>, AppError> {
    let flag = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM educations WHERE id = $1 AND user_id = $2",
    )
    .bind(payload)
    .bind(user.id)
    .fetch_one(&*state.db)
    .await
    .map_err(|_| AppError::InternalError)
    .map_or_else(|_| None, |count| Some(count.0 > 0));

    if !flag.unwrap() {
        return Err(AppError::NotFound {
            error: "Education not found".to_string(),
        });
    }

    Ok(flag)
}
