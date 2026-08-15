mod session;

use crate::auth::session::AdminSession;

pub(crate) enum AuthState {
    Normal,
    Admin(AdminSession),
}
