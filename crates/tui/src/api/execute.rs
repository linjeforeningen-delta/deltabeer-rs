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
            match api.user(&identifier).await {
                Ok(user) => Message::UserLoaded(user),
                Err(error) => Message::UserLoadFailed(error.to_string()),
            }
        }
    }
}