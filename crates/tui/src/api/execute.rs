use crate::api::models::auth::Credentials;
use crate::app::{AuthenticationMessage, TransactionMessage, UserMessage};
use crate::{
    api::client::ApiClient,
    app::{Command, Message},
};

pub(crate) async fn execute_command(
    api: &ApiClient,
    command: Command,
) -> Message {
    match command {
        Command::LookupUser(identifier) => {
            let user_id = match api.resolve_user(&identifier).await {
                Ok(user_id) => user_id,
                Err(error) => {
                    return Message::User(UserMessage::LoadFailed(error.to_string()));
                }
            };

            match api.user(user_id).await {
                Ok(user) => Message::User(UserMessage::Loaded(user)),
                Err(error) => Message::User(UserMessage::LoadFailed(error.to_string())),
            }
        }

        Command::Spend { user_id, amount } => {
            match api.spend(&user_id, amount).await {
                Ok(transaction) => Message::Transaction(TransactionMessage::SpendSuccess(transaction)),
                Err(error) => Message::Transaction(TransactionMessage::SpendFailed(error.to_string())),
            }
        }

        Command::RequestAdminAuth { identifier, password } => {
            let user_id = match api.resolve_user(&identifier).await {
                Ok(user_id) => user_id,
                Err(error) => {
                    return Message::User(UserMessage::LoadFailed(error.to_string()));
                }
            };

            match api.request_admin_token(&Credentials { user_id, password }).await {
                Ok(token) => Message::Authentication(AuthenticationMessage::SingleUseToken(token)),
                Err(error) => Message::Authentication(AuthenticationMessage::AdminAuthFailed(error.to_string())),
            }
        }
    }
}