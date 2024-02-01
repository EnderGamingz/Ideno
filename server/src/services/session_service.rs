use tower_sessions::Session;

use crate::response::error_handling::AppError;

#[derive(Clone)]
pub struct SessionService;

impl SessionService {
    pub async fn get_session_id(session: &Session) -> Option<String> {
        let id = session.get::<String>("user_id").await;

        id.unwrap_or_else(|_| None)
    }

    pub async fn check_logged_in(session: &Session) -> Result<String, AppError> {
        let id = SessionService::get_session_id(session).await;
        match id {
            Some(user) => Ok(user),
            None => return Err(AppError::NotLoggedIn)?,
        }
    }

    pub async fn flush_session(session: &Session) {
        session.flush().await.unwrap();
    }
}
