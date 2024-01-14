use axum::extract::{Path, State};
use axum::response::IntoResponse;
use tower_sessions::Session;

use crate::auth::check_admin::check_admin;
use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

pub async fn admin_delete_education(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    check_admin(&session, &state.db).await?;
    education_exists(&state, id).await?;

    sqlx::query("DELETE FROM educations WHERE id = $1")
        .bind(id)
        .execute(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)?;

    Ok(AppSuccess::DELETED)
}

async fn education_exists(state: &AppState, payload: i32) -> Result<Option<bool>, AppError> {
    let flag = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM educations WHERE id = $1")
        .bind(payload)
        .fetch_one(&*state.db)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));

    if !flag.unwrap() {
        return Err(AppError::NotFound {
            error: "Contact information not found".to_string(),
        });
    }

    Ok(flag)
}
