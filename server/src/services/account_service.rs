use crate::models::user::UserModel;
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Deserialize)]
pub struct AccountUpdatePayload {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PasswordUpdatePayload {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterCredentials {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AccountService {
    db_pool: IdenoPool,
}

impl AccountService {
    pub fn new(db_pool: IdenoPool) -> Self {
        AccountService { db_pool }
    }

    /// Asynchronously creates a new user account in the database.
    ///
    /// # Arguments
    ///
    /// * `payload` - A `RegisterCredentials` struct containing the user's registration data.
    /// * `hash` - A string representing the hashed password for the user account.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `UserModel` representing the newly created user account.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the insert operation.
    ///
    pub async fn create_account(
        &self,
        payload: RegisterCredentials,
        hash: String,
    ) -> Result<UserModel, AppError> {
        sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (email, username, password) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(&payload.email)
        .bind(&payload.username)
        .bind(hash)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| return AppError::InternalError)
    }

    /// Asynchronously updates the password for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose password is to be updated.
    /// * `hash` - A string representing the hashed password to be set for the user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the password is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_password(
        &self,
        user_id: i32,
        hash: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
            .bind(hash)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    /// Asynchronously updates the username for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose username is to be updated.
    /// * `username` - The new username to be set for the user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the username is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_username(
        &self,
        user_id: i32,
        username: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET username = $1 WHERE id = $2")
            .bind(username)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    /// Asynchronously updates the email address for a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose email address is to be updated.
    /// * `email` - The new email address to be set for the user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the outcome of the update operation. If the email address is successfully updated,
    /// it returns `Ok(IdenoDBResult)`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError::InternalError` if there is an internal error while executing the update operation.
    ///
    pub async fn update_email(
        &self,
        user_id: i32,
        email: String,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
            .bind(email)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError)
    }

    /// Asynchronously checks if an email address exists in the database.
    ///
    /// # Arguments
    ///
    /// * `email` - A reference to the email address to be checked.
    ///
    /// # Returns
    ///
    /// Returns a boolean value indicating whether the email address exists in the database (`true`) or not (`false`).
    ///
    /// # Errors
    ///
    /// Returns `false` if there is an internal error while executing the query.
    ///
    pub async fn email_exists(&self, email: &String) -> bool {
        let result = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError);

        match result {
            Ok(Some(_row)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    /// Asynchronously checks if a username exists in the database.
    ///
    /// # Arguments
    ///
    /// * `username` - A reference to the username to be checked.
    ///
    /// # Returns
    ///
    /// Returns a boolean value indicating whether the username exists in the database (`true`) or not (`false`).
    ///
    /// # Errors
    ///
    /// Returns `false` if there is an internal error while executing the query.
    ///
    pub async fn username_exists(&self, username: &String) -> bool {
        let result = sqlx::query("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| return AppError::InternalError);

        match result {
            Ok(Some(_row)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }
}
