use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::services::account_service::RegisterCredentials;
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterCredentials>,
) -> Result<impl IntoResponse, AppError> {
    let user_results = state
        .user_service
        .get_user_by_email_or_username(&payload.email, &payload.username)
        .await?;

    if user_results.is_some() {
        return Err(AppError::BadRequest {
            error: Some("User already exists".to_string()),
        })?;
    }

    let password_hash = bcrypt::hash(&payload.password, 12);

    match password_hash {
        Ok(hash) => {
            let user = state.account_service.create_account(payload, hash).await?;
            state.profile_service.create_profile(user.id).await?;
        }
        Err(_) => Err(AppError::InternalError)?,
    };

    Ok(AppSuccess::CREATED { id: None })
}
