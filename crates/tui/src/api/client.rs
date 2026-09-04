use anyhow::{Context, bail};
use chrono::NaiveDate;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::{fs, path::Path};

use crate::api::auth::{AdminTokenDto, Credentials, SessionToken, SingleUseToken};
use crate::app::AppError;
use delta_api::{
    ApiErrorCode, ApiErrorResponse, StatsSummaryDto, TransactionDto, UserCreateRequestDto, UserDto,
    UserIdDto, UserPatchDto,
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
    pub(crate) fn new(base: impl AsRef<str>, ca_cert_path: Option<&Path>) -> anyhow::Result<Self> {
        let parsed = Self::validate_base_url(base.as_ref())?;
        let base = parsed.as_str().trim_end_matches('/').to_string();

        let mut builder = Client::builder();
        if let Some(ca_path) = ca_cert_path {
            let pem = fs::read(ca_path).with_context(|| {
                format!("failed to read CA certificate from {}", ca_path.display())
            })?;
            let certs = reqwest::Certificate::from_pem_bundle(&pem).with_context(|| {
                format!("failed to parse CA certificate from {}", ca_path.display())
            })?;
            if certs.is_empty() {
                bail!("no certificates found in {}", ca_path.display());
            }
            for cert in certs {
                builder = builder.add_root_certificate(cert);
            }
        }

        let http = builder
            .build()
            .context("failed to initialize HTTPS API client")?;

        Ok(Self { http, base })
    }

    pub(crate) fn validate_base_url(url: &str) -> anyhow::Result<reqwest::Url> {
        let parsed = reqwest::Url::parse(url)
            .with_context(|| format!("malformed or invalid API base URL '{url}'"))?;
        if parsed.scheme() != "https" {
            bail!("insecure API base URL '{url}'; scheme must be 'https'");
        }
        if parsed.host_str().is_none() {
            bail!("invalid API base URL '{url}'; missing host");
        }
        Ok(parsed)
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

    pub(crate) async fn stats(&self) -> Result<StatsSummaryDto> {
        self.json(self.request(Method::GET, "/v1/stats/summary"))
            .await
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn rejects_insecure_http_url() {
        let client = ApiClient::new("http://localhost:3000", None);
        assert!(client.is_err());
        let err = client.err().unwrap().to_string();
        assert!(err.contains("scheme must be 'https'"));
    }

    #[test]
    fn rejects_malformed_urls() {
        assert!(ApiClient::new("localhost:3000", None).is_err());
        assert!(ApiClient::new("https://", None).is_err());
        assert!(ApiClient::new("ftp://localhost:3000", None).is_err());
        assert!(ApiClient::new("not a url", None).is_err());
    }

    #[test]
    fn accepts_valid_https_urls() {
        let client = ApiClient::new("https://localhost:3000", None);
        assert!(client.is_ok());
        let client_ip = ApiClient::new("https://127.0.0.1:3000", None);
        assert!(client_ip.is_ok());
    }

    #[test]
    fn loads_ca_cert_file() {
        let cert_key = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let cert_pem = cert_key.cert.pem();

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(cert_pem.as_bytes()).unwrap();

        let client = ApiClient::new("https://localhost:3000", Some(temp_file.path()));
        assert!(client.is_ok());
    }

    #[test]
    fn fails_on_invalid_ca_cert_file() {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(b"not a valid pem").unwrap();

        let client = ApiClient::new("https://localhost:3000", Some(temp_file.path()));
        assert!(client.is_err());
    }
}
