use anyhow::{Context, Result};
use clap::Parser;
use delta_core::domain::AuthPolicy;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[arg(long, default_value = default_config_path())]
    pub config: PathBuf,
}

const fn default_config_path() -> &'static str {
    if cfg!(debug_assertions) {
        "config/development.yaml"
    } else {
        "config/production.yaml"
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ServerConfig {
    pub bind_addr: String,
    pub database_url: String,
    pub database_pool_size: u32,
}
#[derive(Debug, Deserialize)]
pub(crate) struct AuthConfig {
    pub single_use_token_ttl_seconds: i64,
    pub admin_session_ttl_seconds: i64,
}
#[derive(Debug, Deserialize)]
pub(crate) struct LoggingConfig {
    pub filter: String,
}

impl Config {
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yml::from_str(&text).context("failed to parse configuration")
    }

    pub(crate) fn auth_policy(&self) -> AuthPolicy {
        AuthPolicy {
            single_use_token_ttl: chrono::Duration::seconds(self.auth.single_use_token_ttl_seconds),
            admin_session_ttl: chrono::Duration::seconds(self.auth.admin_session_ttl_seconds),
        }
    }
}
