use crate::api::command::ApiCommand;
use crate::api::models::auth::Credentials;
use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::AppError;
use crate::{api::client::ApiClient, app::Message};

pub(crate) async fn execute_command(api: &ApiClient, command: ApiCommand) -> Message {
    match command.request {
        ApiRequest::LookupUser(identifier) => {
            let user_id = match api.resolve_user(&identifier).await {
                Ok(user_id) => user_id,
                Err(error) => {
                    return Message::Failed(AppError::Api(error.to_string()));
                }
            };

            match api.user(user_id).await {
                Ok(user) => Message::ApiResponse(ApiResult::LookupUser(user)),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        ApiRequest::Spend { user_id, amount } => match api.spend(&user_id, amount).await {
            Ok(transaction) => Message::ApiResponse(ApiResult::Spend(transaction)),
            Err(error) => Message::Failed(AppError::Api(error.to_string())),
        },

        ApiRequest::TopUp { user_id, amount } => {
            if let Some(token) = command.authorization {
                match api.top_up(user_id, amount, token).await {
                    Ok(transaction) => Message::ApiResponse(ApiResult::TopUp(transaction)),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.topup").to_string(),
                ))
            }
        }

        ApiRequest::AuthenticateAdmin { user_id, password } => match api
            .request_admin_token(&Credentials { user_id, password })
            .await
        {
            Ok(token) => Message::ApiResponse(ApiResult::AuthenticateAdmin(token)),
            Err(error) => Message::Failed(AppError::Api(error.to_string())),
        },

        ApiRequest::StartAdminSession { user_id, password } => {
            let single_use = match api
                .request_admin_token(&Credentials {
                    user_id: user_id.clone(),
                    password,
                })
                .await
            {
                Ok(token) => token,
                Err(error) => return Message::Failed(AppError::Api(error.to_string())),
            };

            match api.create_session(&single_use).await {
                Ok(token) => Message::ApiResponse(ApiResult::StartAdminSession { user_id, token }),
                Err(error) => Message::Failed(AppError::Api(error.to_string())),
            }
        }

        ApiRequest::EndAdminSession => {
            if let Some(token) = command.authorization {
                match api.logout(token.into()).await {
                    Ok(()) => Message::ApiResponse(ApiResult::EndAdminSession),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.end_session").to_string(),
                ))
            }
        }

        ApiRequest::MakeUser {
            name,
            username,
            program,
            card_number,
            birthdate,
        } => {
            if let Some(token) = command.authorization {
                match api
                    .make_user(name, username, program, card_number, birthdate, token)
                    .await
                {
                    Ok(user) => Message::ApiResponse(ApiResult::MakeUser(user)),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.create_user").to_string(),
                ))
            }
        }

        ApiRequest::UpdateUser { user_id, patch } => {
            if let Some(token) = command.authorization {
                match api.update_user(user_id, patch, token).await {
                    Ok(user) => Message::ApiResponse(ApiResult::UpdateUser(user)),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.update_user").to_string(),
                ))
            }
        }

        ApiRequest::GrantAdmin { user_id, password } => {
            if let Some(token) = command.authorization {
                match api.grant_admin_privileges(user_id, password, token).await {
                    Ok(()) => Message::ApiResponse(ApiResult::GrantAdmin(user_id)),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.grant").to_string(),
                ))
            }
        }

        ApiRequest::RevokeAdmin { user_id } => {
            if let Some(token) = command.authorization {
                match api.revoke_admin_privileges(user_id, token).await {
                    Ok(()) => Message::ApiResponse(ApiResult::RevokeAdmin(user_id)),
                    Err(error) => Message::Failed(AppError::Api(error.to_string())),
                }
            } else {
                Message::Failed(AppError::Authentication(
                    t!("auth_errors.revoke").to_string(),
                ))
            }
        }
    }
}
