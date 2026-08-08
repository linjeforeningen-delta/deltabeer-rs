use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, NaiveDate, Utc};
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    pub base: String,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: String,
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: u32,
    pub role: Role,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: Amount,
    pub spent: Amount,
}
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Admin,
    User,
}
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Admin => "Admin",
                Self::User => "User",
            }
        )
    }
}
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Amount(pub u32);
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: TransactionKind,
    pub amount: Amount,
    pub timestamp: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub source: TransactionSource,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionKind {
    Spend,
    TopUp,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionSource {
    Live,
    Migration,
    Adjustment,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
    pub total_transactions: u32,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_id: Uuid,
    pub password: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: String,
    pub birthdate: NaiveDate,
}
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PatchUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub program: Option<String>,
    pub card_number: Option<String>,
    pub comments: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RoleBody {
    Admin,
    User,
}

impl ApiClient {
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base: base.into().trim_end_matches('/').to_string(),
            token: None,
        }
    }
    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let req = self.http.request(method, format!("{}{}", self.base, path));
        match &self.token {
            Some(token) => req.bearer_auth(token),
            None => req,
        }
    }
    async fn json<T: for<'de> Deserialize<'de>>(&self, req: RequestBuilder) -> Result<T> {
        let response = req.send().await.context("request failed")?;
        let status = response.status();
        if !status.is_success() {
            return Err(Self::api_error(status, response).await);
        }
        response.json().await.context("invalid API response")
    }
    async fn empty(&self, req: RequestBuilder) -> Result<()> {
        let response = req.send().await.context("request failed")?;
        if !response.status().is_success() {
            return Err(Self::api_error(response.status(), response).await);
        }
        Ok(())
    }
    async fn api_error(status: StatusCode, response: reqwest::Response) -> anyhow::Error {
        let body = response.json::<ErrorBody>().await.ok();
        anyhow!(
            "HTTP {}: {}",
            status,
            body.and_then(|b| b.message.map(|m| format!("{} ({})", m, b.code)))
                .unwrap_or_else(|| "API request failed".into())
        )
    }
    pub async fn login(&mut self, user_id: Uuid, password: String) -> Result<()> {
        let pass_token: String = self
            .json(
                self.request(Method::POST, "/v1/admins/pass")
                    .json(&Credentials { user_id, password }),
            )
            .await?;
        self.token = Some(pass_token);
        let session_token = self.session().await?;
        self.token = Some(session_token);
        Ok(())
    }
    pub async fn session(&self) -> Result<String> {
        self.json(self.request(Method::POST, "/v1/admins/session"))
            .await
    }
    pub async fn logout(&mut self) -> Result<()> {
        self.empty(self.request(Method::DELETE, "/v1/admins/session"))
            .await?;
        self.token = None;
        Ok(())
    }
    pub async fn users(&self) -> Result<Vec<User>> {
        self.json(self.request(Method::GET, "/v1/users")).await
    }
    pub async fn admins(&self) -> Result<Vec<User>> {
        self.json(self.request(Method::GET, "/v1/admins")).await
    }
    pub async fn user(&self, ident: &str) -> Result<User> {
        self.json(self.request(Method::GET, &format!("/v1/users/{ident}")))
            .await
    }
    pub async fn resolve(&self, ident: &str) -> Result<Uuid> {
        self.json(self.request(Method::GET, &format!("/v1/users/resolve/{ident}")))
            .await
    }
    pub async fn spend(&self, ident: &str, amount: u32) -> Result<Transaction> {
        self.json(
            self.request(Method::POST, &format!("/v1/users/{ident}/spend"))
                .json(&amount),
        )
        .await
    }
    pub async fn stats(&self) -> Result<Stats> {
        self.json(self.request(Method::GET, "/v1/stats/")).await
    }
    pub async fn summary(&self) -> Result<Summary> {
        self.json(self.request(Method::GET, "/v1/stats/summary"))
            .await
    }
    pub async fn create_user(&self, body: CreateUser) -> Result<User> {
        self.json(
            self.request(Method::POST, "/v1/admins/user_management/create")
                .json(&body),
        )
        .await
    }
    pub async fn update_user(&self, ident: &str, body: PatchUser) -> Result<User> {
        self.json(
            self.request(
                Method::PATCH,
                &format!("/v1/admins/user_management/{ident}/update"),
            )
            .json(&body),
        )
        .await
    }
    pub async fn topup(&self, ident: &str, amount: u32) -> Result<Transaction> {
        self.json(
            self.request(
                Method::POST,
                &format!("/v1/admins/user_management/{ident}/topup"),
            )
            .json(&amount),
        )
        .await
    }
    pub async fn role(&self, ident: &str, role: RoleBody) -> Result<User> {
        self.json(
            self.request(
                Method::PATCH,
                &format!("/v1/admins/user_management/{ident}/role"),
            )
            .json(&role),
        )
        .await
    }
}
