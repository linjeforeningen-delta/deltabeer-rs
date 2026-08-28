use delta_core::{
    domain::AuthPolicy,
    ports::{
        AdminRepo, Clock, IdGenerator, StatsRepo, TokenRepo, TokenSource, TransactionRepo, UserRepo,
    },
    services::context::Ctx,
};
use std::sync::Arc;

pub(crate) trait StateRepoBounds:
    AdminRepo + StatsRepo + TransactionRepo + UserRepo + Send + Sync + 'static
{
}

impl<T> StateRepoBounds for T where
    T: AdminRepo + StatsRepo + TransactionRepo + UserRepo + Send + Sync + 'static
{
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) repo: Arc<dyn StateRepoBounds>,
    pub(crate) token_repo: Arc<dyn TokenRepo + Send + Sync>,
    pub(crate) clock: Arc<dyn Clock + Send + Sync>,
    pub(crate) ids: Arc<dyn IdGenerator + Send + Sync>,
    pub(crate) tokens: Arc<dyn TokenSource + Send + Sync>,
    pub(crate) auth_policy: AuthPolicy,
}

impl AppState {
    pub(crate) fn ctx(&self) -> Ctx<'_, dyn StateRepoBounds> {
        Ctx {
            repo: &*self.repo,
            token_repo: &*self.token_repo,
            clock: &*self.clock,
            ids: &*self.ids,
            tokens: &*self.tokens,
        }
    }
}
