use crate::ports::Clock;

pub struct Ctx<'a, R> {
    pub repo: &'a R,
    pub clock: &'a dyn Clock,
}
