use crate::ports::{Clock, IdGenerator, TokenRepo, TokenSource};

/// Dependencies supplied to a core service invocation.
///
/// Keeping these as ports makes service behavior independent of HTTP and SQL.
pub struct Ctx<'a, R: ?Sized> {
    /// Repository port required by the current service operation.
    pub repo: &'a R,
    /// Separate token persistence port used by authentication workflows.
    pub token_repo: &'a (dyn TokenRepo + Sync),
    /// Time source kept injectable so time-dependent rules are testable.
    pub clock: &'a (dyn Clock + Sync),
    /// Identifier source kept outside the domain workflow.
    pub ids: &'a (dyn IdGenerator + Sync),
    /// Token implementation responsible for issuing and validating opaque tokens.
    pub tokens: &'a (dyn TokenSource + Sync),
}
