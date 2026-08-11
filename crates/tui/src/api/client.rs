use anyhow::{Context, Result, anyhow};
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use crate::api::models::
auth::SessionToken;
use crate::api::models::auth::{AdminToken, Credentials, SingleUseToken};
use crate::api::models::transaction::Transaction;
use crate::api::models::user::{User, UserId};

#[derive(Clone)]
pub(crate) struct ApiClient {
    http: Client,
    base: String,
    session_token: Option<SessionToken>,
}

impl ApiClient {
    pub(crate) fn new(base: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base: base.into().trim_end_matches('/').to_string(),
            session_token: None,
        }
    }

    pub(crate) fn set_session_token(
        &mut self,
        token: SessionToken,
    ) {
        self.session_token = Some(token);
    }

    pub(crate) fn clear_session_token(&mut self) {
        self.session_token = None;
    }

    pub(crate) fn has_session(&self) -> bool {
        self.session_token.is_some()
    }

    fn request(
        &self,
        method: Method,
        path: &str,
    ) -> RequestBuilder {
        let request = self
            .http
            .request(method, format!("{}{}", self.base, path));

        match &self.session_token {
            Some(token) => {
                request.bearer_auth(token.as_str())
            }

            None => request,
        }
    }

    async fn json<T>(
        &self,
        request: RequestBuilder,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request
            .send()
            .await
            .context("API request failed")?;

        let status = response.status();

        if !status.is_success() {
            return Err(Self::api_error(status, response).await);
        }

        response
            .json::<T>()
            .await
            .context("Invalid API response")
    }

    async fn empty(
        &self,
        request: RequestBuilder,
    ) -> Result<()> {
        let response = request
            .send()
            .await
            .context("API request failed")?;

        let status = response.status();

        if !status.is_success() {
            return Err(Self::api_error(status, response).await);
        }

        Ok(())
    }

    async fn api_error(
        status: StatusCode,
        response: reqwest::Response,
    ) -> anyhow::Error {
        let text = response.text().await.ok();

        match text {
            Some(body) if !body.is_empty() => {
                anyhow!("HTTP {status}: {body}")
            }

            _ => {
                anyhow!("HTTP {status}: API request failed")
            }
        }
    }
}

impl ApiClient {
    pub(crate) async fn user(
        &self,
        user_id: UserId,
    ) -> Result<User> {
        self.json(
            self.request(
                Method::GET,
                &format!("/v1/users/{user_id}"),
            ),
        )
            .await
    }

    pub(crate) async fn resolve_user(
        &self,
        identifier: &str,
    ) -> Result<UserId> {
        self.json(
            self.request(
                Method::GET,
                &format!("/v1/users/resolve/{identifier}"),
            ),
        )
            .await
    }

    pub(crate) async fn users(&self) -> Result<Vec<User>> {
        self.json(
            self.request(Method::GET, "/v1/users"),
        )
            .await
    }

    pub(crate) async fn spend(
        &self,
        user_id: &UserId,
        amount: u32,
    ) -> Result<Transaction> {
        self.json(
            self.request(
                Method::POST,
                &format!("/v1/users/{user_id}/spend"),
            )
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
        let admin_token: AdminToken = self.json(
            self.request(Method::POST, "/v1/admins/pass")
                .json(credentials)
                .into(),
        )
            .await?;

        Ok(admin_token.into())
    }

    pub(crate) async fn create_session(
        &self,
        token: &SingleUseToken,
    ) -> Result<SessionToken> {
        let admin_token: AdminToken = self
            .json(
                self.http
                    .post(format!("{}/v1/admins/session", self.base))
                    .bearer_auth(token.as_str()),
            )
            .await?;

        Ok(admin_token.into())
    }

    pub(crate) async fn logout(&self) -> Result<()> {
        self.empty(
            self.request(
                Method::DELETE,
                "/v1/admins/session",
            ),
        )
            .await
    }
}

pub(crate) enum Authorization<'a> {
    Session,
    SingleUse(&'a SingleUseToken),
}


impl ApiClient {
    pub(crate) async fn top_up(
        &self,
        identifier: &str,
        amount: u32,
        authorization: Authorization<'_>,
    ) -> Result<Transaction> {
        let request = self
            .http
            .post(format!(
                "{}/v1/admins/user_management/{identifier}/topup",
                self.base
            ))
            .json(&amount);

        let request = match authorization {
            Authorization::Session => {
                match &self.session_token {
                    Some(token) => {
                        request.bearer_auth(token.as_str())
                    }

                    None => {
                        return Err(anyhow!(
                            "No admin session is active"
                        ));
                    }
                }
            }

            Authorization::SingleUse(token) => {
                request.bearer_auth(token.as_str())
            }
        };

        self.json(request).await
    }
}