use crate::ports::{Clock, IdGenerator, TokenSource};

pub struct Ctx<'a, R: ?Sized> {
    pub repo: &'a R,
    pub clock: &'a (dyn Clock + Sync),
    pub ids: &'a (dyn IdGenerator + Sync),
    pub tokens: &'a (dyn TokenSource + Sync),
}
