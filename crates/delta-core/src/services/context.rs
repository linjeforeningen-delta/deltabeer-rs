use crate::ports::Clock;
use crate::ports::IdGenerator;

pub struct Ctx<'a, R> {
    pub repo: &'a R,
    pub clock: &'a dyn Clock,
    pub ids: &'a dyn IdGenerator,
}
