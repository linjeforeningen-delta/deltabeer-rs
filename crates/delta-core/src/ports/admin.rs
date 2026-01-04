use crate::domain::{ActionRecord, AdminGrantId, UserId};
use crate::ports::RepoError;
use async_trait::async_trait;

pub struct Admin {
    id: String,
    password_hash: String,
}

#[async_trait]
pub trait AdminRepo {
    async fn get_admin(&self, id: UserId) -> Result<String, RepoError>;
    async fn grant_admin(
        &self,
        admin_grant_id: AdminGrantId,
        user_id: UserId,
        data: String,
        record: ActionRecord,
    ) -> Result<(), RepoError>;
    async fn revoke_admin(&self, id: UserId, record: ActionRecord) -> Result<(), RepoError>;
}
