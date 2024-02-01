use axum::extract::{Path, State};
use tower_sessions::Session;

use crate::response::error_handling::AppError;
use crate::response::success_handling::AppSuccess;
use crate::AppState;

pub async fn admin_delete_contact_information(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i32>,
) -> Result<AppSuccess, AppError> {
    state.user_service.check_admin(&session).await?;
    state
        .contact_information_service
        .contact_info_exists(id)
        .await?;

    state
        .contact_information_service
        .admin_delete_contact_information(id)
        .await?;

    Ok(AppSuccess::DELETED)
}
