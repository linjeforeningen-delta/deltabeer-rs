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
                    return Message::UserLoadFailed(error.to_string());
                }
            };

            match api.user(user_id).await {
                Ok(user) => Message::UserLoaded(user),
                Err(error) => Message::UserLoadFailed(error.to_string()),
            }
        }

        Command::Spend { user_id, amount } => {
            match api.spend(&user_id, amount).await {
                Ok(transaction) => Message::SpendSuccess(transaction),
                Err(error) => Message::SpendFailed(error.to_string()),
            }
        }
    }
}