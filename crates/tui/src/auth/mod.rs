mod context;
mod session;

pub(crate) use context::AdminContext;
pub(crate) use session::AdminSession;

pub(crate) enum AuthState {
    Normal,
    Admin(AdminSession),
}
