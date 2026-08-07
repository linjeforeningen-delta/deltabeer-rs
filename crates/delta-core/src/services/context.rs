use crate::ports::{Clock, IdGenerator, TokenRepo, TokenSource};

pub struct Ctx<'a, R: ?Sized> {
    pub repo: &'a R,
    pub token_repo: &'a (dyn TokenRepo + Sync),
    pub clock: &'a (dyn Clock + Sync),
    pub ids: &'a (dyn IdGenerator + Sync),
    pub tokens: &'a (dyn TokenSource + Sync),
}
