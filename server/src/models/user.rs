use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct PublicUserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}
