use sqlx::FromRow;
use serde::Serialize;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}
