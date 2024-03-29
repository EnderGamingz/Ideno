use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{IdenoDBResult, IdenoPool};
use crate::models::user::{UserModel};
use crate::response::error_handling::AppError;
use crate::services::session_service::SessionService;

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone)]
pub struct UserService {
    db_pool: IdenoPool,
}

impl UserService {
    pub fn new(db_pool: IdenoPool) -> Self {
        UserService { db_pool }
    }

    /// Asynchronously checks if the session corresponds to an admin user.
    ///
    /// # Arguments
    ///
    /// * `session` - A reference to the session to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `UserModel` if the session corresponds to an admin user,
    /// otherwise returns an `AppError::Forbidden` error.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::Forbidden` error if the user is not an admin.
    ///
    pub async fn check_admin(&self, session: &Session) -> Result<UserModel, AppError> {
        let user = self.check_user(&session).await?;

        match user.role.as_str() {
            "admin" => Ok(user),
            _ => Err(AppError::Forbidden { error: None }),
        }
    }

    /// Asynchronously checks if the session corresponds to a logged-in user and retrieves the user data.
    ///
    /// # Arguments
    ///
    /// * `session` - A reference to the session to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `UserModel` if the session corresponds to a logged-in user
    /// and the user data is found, otherwise returns an `AppError`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if the session is not valid, or if the user data is not found.
    ///
    pub async fn check_user(&self, session: &Session) -> Result<UserModel, AppError> {
        let user_id = SessionService::check_logged_in(session).await?;
        let user_result = self.get_user(user_id).await?;

        match user_result {
            Some(user) => Ok(user),
            None => Err(AppError::UserNotFound)?,
        }
    }

    /// Asynchronously retrieves a user by email or username from the database.
    ///
    /// # Arguments
    ///
    /// * `email` - A reference to the email of the user to retrieve.
    /// * `username` - A reference to the username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<UserModel>`. If a user with the specified email or username
    /// is found in the database, it returns `Some(UserModel)` with the user data. If no user is found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if there is an internal error while querying the database.
    ///
    pub async fn get_user_by_email_or_username(
        &self,
        email: &String,
        username: &String,
    ) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1 OR email = $2")
            .bind(username)
            .bind(email)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves a user by either email or username from the database.
    ///
    /// # Arguments
    ///
    /// * `email_or_username` - The email or username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<UserModel>`. If a user with the specified email or username
    /// is found in the database, it returns `Some(UserModel)` with the user data. If no user is found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if there is an internal error while querying the database.
    ///
    pub async fn get_user_by_either_email_or_username(
        &self,
        email_or_username: String,
    ) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(email_or_username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves an authenticated user's data by user ID from the database.
    ///
    /// This function is intended to retrieve data specifically for authenticated users.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the authenticated user whose data is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `UserModel` if the user with the specified ID is found in the database.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::UserNotFound` error if no user with the specified ID is found in the database.
    ///
    pub async fn get_auth_user(
        &self,
        user_id: String,
    ) -> Result<UserModel, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::UserNotFound)
    }

    /// Asynchronously checks if the session corresponds to a logged-in user and retrieves the user data if available.
    ///
    /// This function is similar to `check_user`, but it returns an `Option<UserModel>`, allowing for cases where the user is not logged in.
    ///
    /// # Arguments
    ///
    /// * `session` - A reference to the session to be checked.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<UserModel>`. If the session corresponds to a logged-in user,
    /// it returns `Some(UserModel)` with the user data. If the session does not correspond to a logged-in user, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if there is an internal error while retrieving the user data.
    ///
    pub async fn check_user_optional(
        &self,
        session: &Session,
    ) -> Result<Option<UserModel>, AppError> {
        let user_id = SessionService::get_session_id(session).await;

        match user_id {
            Some(user) => self.get_user(user).await,
            None => Ok(None),
        }
    }

    /// Asynchronously retrieves a user by user ID from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<UserModel>`. If a user with the specified ID is found in the database,
    /// it returns `Some(UserModel)` with the user data. If no user is found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if there is an internal error while querying the database.
    ///
    pub async fn get_user(&self, user_id: String) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves a user by username from the database.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `UserModel` if a user with the specified username is found in the database.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::UserNotFound` error if no user with the specified username is found in the database,
    /// or returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_user_by_username(&self, username: String) -> Result<UserModel, AppError> {
        let user = sqlx::query_as::<_, UserModel>("SELECT * FROM users where username = ?")
            .bind(username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)?;
        match user {
            Some(user) => Ok(user),
            None => Err(AppError::UserNotFound)?,
        }
    }

    /// Asynchronously deletes a user from the database by user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to delete.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the delete operation. If the user is successfully deleted,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the delete operation.
    ///
    pub async fn delete_user(&self, user_id: i32) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously updates a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to update.
    /// * `payload` - An `UpdateUserRequest` containing the new data for the user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the user is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_user(
        &self,
        user_id: i32,
        payload: UpdateUserRequest,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET username = $1, email = $2, role = $3 WHERE id = $4")
            .bind(payload.username)
            .bind(payload.email)
            .bind(payload.role)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    /// Asynchronously retrieves user data by user ID from the database for administrative purposes.
    ///
    /// This function is intended for administrative use and may return sensitive user information.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<UserModel>`. If a user with the specified ID is found in the database,
    /// it returns `Some(UserModel)` with the user data. If no user is found, it returns `None`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    /// Logs an error using `tracing::error!` and returns `AppError::InternalError` if there is an error while fetching the user data.
    ///
    pub async fn admin_get_user(&self, user_id: i32) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ? LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|err| {
                tracing::error!("Failed to fetch user: {}", err);
                AppError::InternalError
            })
    }

    /// Asynchronously retrieves all user data from the database for administrative purposes.
    ///
    /// This function is intended for administrative use and retrieves data for all users.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `UserModel` instances representing all users in the database.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while querying the database.
    ///
    pub async fn get_all_users(&self) -> Result<Vec<UserModel>, AppError> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }
}
