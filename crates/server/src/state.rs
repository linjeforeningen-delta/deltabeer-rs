use delta_core::ports::{
    AdminRepo, Clock, IdGenerator, TokenRepo, TokenSource, TransactionRepo, UserRepo,
};
use delta_core::services::context::Ctx;
use std::sync::Arc;

pub(crate) trait StateRepoBounds:
    AdminRepo + TransactionRepo + UserRepo + Send + Sync + 'static
{
}

impl<T> StateRepoBounds for T where T: AdminRepo + TransactionRepo + UserRepo + Send + Sync + 'static
{}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn StateRepoBounds>,
    pub token_repo: Arc<dyn TokenRepo + Send + Sync>,
    pub clock: Arc<dyn Clock + Send + Sync>,
    pub ids: Arc<dyn IdGenerator + Send + Sync>,
    pub tokens: Arc<dyn TokenSource + Send + Sync>,
}

impl AppState {
    pub fn ctx(&self) -> Ctx<'_, dyn StateRepoBounds> {
        Ctx {
            repo: &*self.repo,
            token_repo: &*self.token_repo,
            clock: &*self.clock,
            ids: &*self.ids,
            tokens: &*self.tokens,
        }
    }
}
