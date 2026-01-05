use crate::ports::{Clock, IdGenerator, TokenSource};

pub struct Ctx<'a, R> {
    pub repo: &'a R,
    pub clock: &'a dyn Clock,
    pub ids: &'a dyn IdGenerator,
    pub tokens: &'a dyn TokenSource,
}
