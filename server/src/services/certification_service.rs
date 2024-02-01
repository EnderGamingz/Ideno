use crate::models::certification::{
    AuthCertificationModel, CertificationModel, PublicCertificationModel,
};
use crate::response::error_handling::AppError;
use crate::{IdenoDBResult, IdenoPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddCertificationPayload {
    pub name: String,
    pub organization: String,
    pub issue_date: Option<String>,
    pub expiration_date: Option<String>,
    pub credential_id: Option<String>,
    pub credential_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateCertificationPayload {
    name: String,
    organization: String,
    issue_date: Option<String>,
    expiration_date: Option<String>,
    credential_id: Option<String>,
    credential_url: Option<String>,
}

#[derive(Clone)]
pub struct CertificationService {
    db_pool: IdenoPool,
}

impl CertificationService {
    pub fn new(db_pool: IdenoPool) -> Self {
        CertificationService { db_pool }
    }

    pub async fn get_authenticated_certifications(
        &self,
        user_id: i32,
    ) -> Result<Vec<AuthCertificationModel>, AppError> {
        sqlx::query_as::<_, AuthCertificationModel>(
            "SELECT
                id,
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn get_public_certifications(
        &self,
        user_id: i32,
    ) -> Result<Vec<PublicCertificationModel>, AppError> {
        sqlx::query_as::<_, PublicCertificationModel>(
            "SELECT
                name,
                organization,
                issue_date,
                expiration_date,
                credential_id,
                credential_url
              FROM certification
              WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
    }

    pub async fn user_owns_certification(
        &self,
        user_id: i32,
        certification_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM certification WHERE id = $1 AND user_id = $2",
        )
        .bind(certification_id)
        .bind(user_id)
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_| AppError::InternalError)
        .map_or_else(|_| None, |count| Some(count.0 > 0));
        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Certification not found".to_string(),
            }),
        }
    }

    pub async fn delete_certification(
        &self,
        user_id: i32,
        certification_id: i32,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("DELETE FROM certification WHERE id = $1 AND user_id = $2")
            .bind(certification_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn update_certification(
        &self,
        user_id: i32,
        certification_id: i32,
        payload: UpdateCertificationPayload,
    ) -> Result<IdenoDBResult, AppError> {
        sqlx::query("UPDATE certification SET name = $1, organization = $2, issue_date = $3, expiration_date = $4, credential_id = $5, credential_url = $6 WHERE id = $7 AND user_id = $8")
            .bind(payload.name)
            .bind(payload.organization)
            .bind(payload.issue_date)
            .bind(payload.expiration_date)
            .bind(payload.credential_id)
            .bind(payload.credential_url)
            .bind(certification_id)
            .bind(user_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn create_certification(
        &self,
        user_id: i32,
        payload: AddCertificationPayload,
    ) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64, )>
            ("INSERT INTO certification (user_id, name, organization, issue_date, expiration_date, credential_id, credential_url) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
            .bind(user_id)
            .bind(payload.name)
            .bind(payload.organization)
            .bind(payload.issue_date)
            .bind(payload.expiration_date)
            .bind(payload.credential_id)
            .bind(payload.credential_url)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|id| id.0)
    }

    pub async fn get_certification_count(&self, user_id: i32) -> Result<i64, AppError> {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM certification WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|count| count.0)
    }

    pub async fn get_all_certifications(
        &self,
        user_id: i32,
    ) -> Result<Vec<CertificationModel>, AppError> {
        sqlx::query_as::<_, CertificationModel>("SELECT * FROM certification WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn admin_delete_certification(&self, certification_id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM certification WHERE id = $1")
            .bind(certification_id)
            .execute(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map(|_| ())
    }

    pub async fn certification_exists(
        &self,
        certification_id: i32,
    ) -> Result<Option<bool>, AppError> {
        let flag = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM certification WHERE id = $1")
            .bind(certification_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|_| AppError::InternalError)
            .map_or_else(|_| None, |count| Some(count.0 > 0));

        match flag {
            Some(flag) => Ok(Some(flag)),
            None => Err(AppError::NotFound {
                error: "Certification not found".to_string(),
            }),
        }
    }
}
