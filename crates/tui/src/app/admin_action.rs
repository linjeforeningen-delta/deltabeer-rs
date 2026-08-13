use crate::api::models::auth::AdminToken;
use crate::api::models::user::UserId;
use crate::app::Command;


#[derive(Debug)]
pub(crate) enum AdminAction {
    TopUp {
        user_id: UserId,
        amount: u32,
    },
}

impl AdminAction {
    pub(crate) fn into_command(
        self,
        token: AdminToken,
    ) -> Command {
        match self {
            Self::TopUp { user_id, amount } => {
                Command::TopUp {
                    user_id,
                    amount,
                    token,
                }
            }
        }
    }
}