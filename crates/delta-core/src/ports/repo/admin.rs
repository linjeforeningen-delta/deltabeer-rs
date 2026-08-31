use crate::{
    domain::{ActionRecord, AdminGrantId, PasswordHash, UserId},
    ports::repo::RepoError,
};
use async_trait::async_trait;

/// Persistence operations for administrator credentials and grants.
///
/// Implementations own storage concerns, while authorization workflows and
/// password hashing remain in the service layer.
#[async_trait]
pub trait AdminRepo {
    async fn get_admin(&self, id: UserId) -> Result<PasswordHash, RepoError>;
    async fn grant_admin(
        &self,
        admin_grant_id: AdminGrantId,
        user_id: UserId,
        password_hash: PasswordHash,
        record: ActionRecord,
    ) -> Result<(), RepoError>;
    async fn revoke_admin(&self, id: UserId, record: ActionRecord) -> Result<(), RepoError>;

    async fn update_admin_password(
        &self,
        id: UserId,
        password_hash: PasswordHash,
    ) -> Result<(), RepoError>;
}
