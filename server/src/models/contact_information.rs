use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct ContactInformationModel {
    pub id: i32,
    pub user_id: i32,
    pub type_field: String,
    pub value: String,
    pub created_at: String,
}
