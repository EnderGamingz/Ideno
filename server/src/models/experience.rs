use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct ExperienceModel {
    pub id: i32,
    pub user_id: i32,
    pub company: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub exp_type: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
