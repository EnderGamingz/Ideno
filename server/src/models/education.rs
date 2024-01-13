use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct EducationModel {
    pub id: i32,
    pub user_id: i32,
    pub school: String,
    pub degree: String,
    pub field: String,
    pub start_date: String,
    pub end_date: String,
    pub created_at: String,
}
