use crate::api::models::auth::Credentials;
use crate::app::AppError;
use crate::app::message::RequestResult;
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
                    return Message::Failed(AppError::Api(error.to_string()));
                }
            };

            match api.user(user_id).await {
                Ok(user) => Message::Response(RequestResult::UserLoaded(user)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        Command::Spend { user_id, amount } => {
            match api.spend(&user_id, amount).await {
                Ok(transaction) => Message::Response(RequestResult::SpendSucceeded(transaction)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        Command::TopUp { user_id, amount, token } => {
            match api.top_up(user_id, amount, token).await {
                Ok(transaction) => Message::Response(RequestResult::TopUpSucceeded(transaction)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        Command::RequestAdminAuth { identifier, password } => {
            let user_id = match api.resolve_user(&identifier).await {
                Ok(user_id) => user_id,
                Err(error) => {
                    return Message::Failed(AppError::Api(error.to_string()));
                }
            };

            match api.request_admin_token(&Credentials { user_id, password }).await {
                Ok(token) => Message::Response(RequestResult::AdminAuthenticated(token)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        Command::MakeUser {
            name,
            username,
            program,
            card_number,
            birthdate,
            token,
        } => {
            match api.make_user(name, username, program, card_number, birthdate, token).await {
                Ok(user) => Message::Response(RequestResult::MakeUserSucceeded(user)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }
    }
}