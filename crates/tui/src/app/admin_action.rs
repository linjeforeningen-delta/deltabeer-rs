use crate::api::models::auth::AdminToken;
use crate::api::models::user::UserId;
use crate::app::Command;
use chrono::NaiveDate;


#[derive(Debug)]
pub(crate) enum AdminAction {
    TopUp {
        user_id: UserId,
        amount: u32,
    },

    MakeUser {
        name: String,
        username: String,
        program: String,
        card_number: u32,
        birthdate: NaiveDate,
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

            Self::MakeUser {
                name,
                username,
                program,
                card_number,
                birthdate,
            } => {
                Command::MakeUser {
                    name,
                    username,
                    program,
                    card_number,
                    birthdate,
                    token,
                }
            }
        }
    }
}