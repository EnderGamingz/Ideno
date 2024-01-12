use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct ProfileModel {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub pronouns: String,
    pub headline: String,
    pub country: String,
    pub city: String,
    pub bio: String,
    pub created_at: String,
    pub updated_at: String,
}
