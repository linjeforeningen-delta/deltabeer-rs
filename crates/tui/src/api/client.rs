use chrono::NaiveDate;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use crate::api::auth::{AdminTokenDto, Credentials, SessionToken, SingleUseToken};
use crate::app::AppError;
use delta_api::{
    ApiErrorCode, ApiErrorResponse, TransactionDto, UserCreateRequestDto, UserDto, UserIdDto,
    UserPatchDto,
};

#[derive(Debug)]
pub(crate) enum ApiClientError {
    Transport {
        message: String,
    },
    Api {
        status: StatusCode,
        code: ApiErrorCode,
        message: Option<String>,
    },
    InvalidResponse {
        message: String,
    },
}

impl From<ApiClientError> for AppError {
    fn from(error: ApiClientError) -> Self {
        match error {
            ApiClientError::Transport { .. } => Self::Transport,
            ApiClientError::InvalidResponse { .. } => Self::InvalidResponse,
            ApiClientError::Api { code, .. } => match code {
                ApiErrorCode::InvalidUserIdentifier => Self::InvalidUserIdentifier,
                ApiErrorCode::BadRequest => Self::BadRequest,
                ApiErrorCode::NotFound => Self::NotFound,
                ApiErrorCode::Conflict => Self::Conflict,
                ApiErrorCode::Unauthorized => Self::Unauthorized,
                ApiErrorCode::Forbidden => Self::Forbidden,
                ApiErrorCode::InternalError => Self::Api,
            },
        }
    }
}

impl std::fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport { message } => write!(f, "API request failed: {message}"),
            Self::Api {
                status,
                code,
                message: Some(message),
            } => write!(f, "HTTP {status} [{code}]: {message}"),
            Self::Api {
                status,
                code,
                message: None,
            } => write!(f, "HTTP {status} [{code}]"),
            Self::InvalidResponse { message } => write!(f, "Invalid API response: {message}"),
        }
    }
}

type Result<T> = std::result::Result<T, ApiClientError>;

#[derive(Clone)]
pub(crate) struct ApiClient {
    http: Client,
    base: String,
}

impl ApiClient {
    pub(crate) fn new(base: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base: base.into().trim_end_matches('/').to_string(),
        }
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        tracing::debug!(%method, path, "preparing API request");
        self.http.request(method, format!("{}{}", self.base, path))
    }

    async fn json<T>(&self, request: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await.map_err(|error| {
            tracing::error!("API request transport failure");
            ApiClientError::Transport {
                message: error.to_string(),
            }
        })?;

        let status = response.status();

        if !status.is_success() {
            tracing::warn!(%status, "API request returned an error status");
            return Err(Self::api_error(status, response).await);
        }

        response.json::<T>().await.map_err(|error| {
            tracing::error!(error = %error, "API response decoding failed");
            ApiClientError::InvalidResponse {
                message: error.to_string(),
            }
        })
    }

    async fn empty(&self, request: RequestBuilder) -> Result<()> {
        let response = request.send().await.map_err(|error| {
            tracing::error!("API request transport failure");
            ApiClientError::Transport {
                message: error.to_string(),
            }
        })?;

        let status = response.status();

        if !status.is_success() {
            tracing::warn!(%status, "API request returned an error status");
            return Err(Self::api_error(status, response).await);
        }

        Ok(())
    }

    async fn api_error(status: StatusCode, response: reqwest::Response) -> ApiClientError {
        match response.json::<ApiErrorResponse>().await {
            Ok(error) => ApiClientError::Api {
                status,
                code: error.code,
                message: error.message,
            },
            Err(error) => ApiClientError::InvalidResponse {
                message: {
                    tracing::error!(error = %error, "API error response decoding failed");
                    error.to_string()
                },
            },
        }
    }
}

impl ApiClient {
    pub(crate) async fn user(&self, user_id: UserIdDto) -> Result<UserDto> {
        self.json(self.request(Method::GET, &format!("/v1/users/{user_id}")))
            .await
    }

    pub(crate) async fn resolve_user(&self, identifier: &str) -> Result<UserIdDto> {
        self.json(self.request(Method::GET, &format!("/v1/users/resolve/{identifier}")))
            .await
    }

    pub(crate) async fn users(&self) -> Result<Vec<UserDto>> {
        self.json(self.request(Method::GET, "/v1/users")).await
    }

    pub(crate) async fn spend(&self, user_id: &UserIdDto, amount: u32) -> Result<TransactionDto> {
        self.json(
            self.request(Method::POST, &format!("/v1/users/{user_id}/spend"))
                .json(&amount),
        )
        .await
    }
}

impl ApiClient {
    pub(crate) async fn request_admin_token(
        &self,
        credentials: &Credentials,
    ) -> Result<SingleUseToken> {
        let admin_token: AdminTokenDto = self
            .json(
                self.request(Method::POST, "/v1/admins/pass")
                    .json(credentials),
            )
            .await?;

        Ok(admin_token.into())
    }

    pub(crate) async fn create_session(&self, token: &SingleUseToken) -> Result<SessionToken> {
        let admin_token: AdminTokenDto = self
            .json(
                self.http
                    .post(format!("{}/v1/admins/session", self.base))
                    .bearer_auth(token.as_str()),
            )
            .await?;

        Ok(admin_token.into())
    }

    pub(crate) async fn logout(&self, token: AdminTokenDto) -> Result<()> {
        let request = self
            .http
            .delete(format!("{}/v1/admins/session", self.base))
            .bearer_auth(token.0.as_str());

        self.empty(request).await
    }
}

impl ApiClient {
    pub(crate) async fn top_up(
        &self,
        user_id: UserIdDto,
        amount: u32,
        token: AdminTokenDto,
    ) -> Result<TransactionDto> {
        let request = self
            .http
            .post(format!(
                "{}/v1/admins/user_management/{user_id}/topup",
                self.base
            ))
            .json(&amount);

        let request = request.bearer_auth(token.0.as_str());

        self.json(request).await
    }

    pub(crate) async fn make_user(
        &self,
        name: String,
        username: String,
        program: String,
        card_number: u32,
        birthdate: NaiveDate,
        token: AdminTokenDto,
    ) -> Result<UserDto> {
        let content = UserCreateRequestDto {
            name,
            username,
            program,
            card_number,
            birthdate,
        };
        let request = self
            .http
            .post(format!("{}/v1/admins/user_management/create", self.base))
            .json(&content)
            .bearer_auth(token.0.as_str());

        self.json(request).await
    }

    pub(crate) async fn update_user(
        &self,
        user_id: UserIdDto,
        patch: UserPatchDto,
        token: AdminTokenDto,
    ) -> Result<UserDto> {
        let request = self
            .http
            .patch(format!(
                "{}/v1/admins/user_management/{user_id}/update",
                self.base
            ))
            .json(&patch)
            .bearer_auth(token.0.as_str());

        self.json(request).await
    }

    pub(crate) async fn grant_admin_privileges(
        &self,
        user_id: UserIdDto,
        password: String,
        token: AdminTokenDto,
    ) -> Result<()> {
        let request = self
            .http
            .post(format!(
                "{}/v1/admins/user_management/{user_id}/admin",
                self.base
            ))
            .json(&password)
            .bearer_auth(token.0.as_str());

        self.empty(request).await
    }

    pub(crate) async fn revoke_admin_privileges(
        &self,
        user_id: UserIdDto,
        token: AdminTokenDto,
    ) -> Result<()> {
        let request = self
            .http
            .delete(format!(
                "{}/v1/admins/user_management/{user_id}/admin",
                self.base
            ))
            .bearer_auth(token.0.as_str());

        self.empty(request).await
    }
}
