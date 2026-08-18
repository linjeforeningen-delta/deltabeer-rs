mod session;
mod context;

use crate::auth::session::AdminSession;

pub(crate) enum AuthState {
    Normal,
    Admin(AdminSession),
}
pub(crate) use context::AdminContext;
