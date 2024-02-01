use crate::models::contact_information::ContactInformationModel;
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddContactInformationPayload {
    pub contact_type: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateContactInformationPayload {
    pub contact_type: String,
    pub value: String,
}

#[derive(Clone)]
pub struct ContactInformationService {
    db_pool: IdenoPool,
}

impl ContactInformationService {
    pub fn new(db_pool: IdenoPool) -> Self {
        ContactInformationService { db_pool }
    }

    pub async fn user_owns_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM contact_information WHERE id = $1 AND user_id = $2",
        )
        .bind(contact_information_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Contact information not found".to_string(),
            }),
        }
    }

    pub async fn delete_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM contact_information WHERE id = $1 AND user_id = $2")
            .bind(contact_information_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn update_contact_information(
        &self,
        user_id: i32,
        contact_information_id: i32,
        payload: UpdateContactInformationPayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query(
            "UPDATE contact_information SET type_field = $1, value = $2 WHERE id = $3 AND user_id = $4",
        )
            .bind(payload.contact_type)
            .bind(payload.value)
            .bind(contact_information_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn create_contact_information(
        &self,
        user_id: i32,
        payload: AddContactInformationPayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("INSERT INTO contact_information (user_id, type_field, value) VALUES ($1, $2, $3) RETURNING id")
            .bind(user_id)
            .bind(payload.contact_type)
            .bind(payload.value)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    pub async fn get_existing_contact_information(
        &self,
        user_id: i32,
        data: &AddContactInformationPayload,
    ) -> Result<bool, AppError> {
        sqlx::query_as::<_, ContactInformationModel>(
            "SELECT COUNT(*) FROM contact_information WHERE user_id = $1 AND type_field = $2 AND value = $3 LIMIT 1",
        )
            .bind(user_id)
            .bind(&data.contact_type)
            .bind(&data.value)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.len() > 0)
    }

    pub async fn get_contact_information_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    pub async fn get_all_contact_information(
        &self,
        user_id: i32,
    ) -> Result<Vec<ContactInformationModel>, AppError> {
        sqlx::query_as::<_, ContactInformationModel>(
            "SELECT * FROM contact_information WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn admin_delete_contact_information(
        &self,
        contact_information_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query("DELETE FROM contact_information WHERE id = $1")
            .bind(contact_information_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    pub async fn contact_info_exists(
        &self,
        contact_information_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM contact_information WHERE id = $1")
                .bind(contact_information_id)
                .fetch_one(&self.db_pool)
                .await
                .map_err(|_| AppError::InternalError)
                .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Contact information not found".to_string(),
            }),
        }
    }
}
