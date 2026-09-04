use anyhow::{Context, Result, bail};
use clap::Parser;
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
    pub tui: TuiConfig,
}
#[derive(Debug, Deserialize)]
pub(crate) struct TuiConfig {
    pub api_base_url: String,
    pub event_poll_interval_ms: u64,
    pub scanner_max_gap_ms: u64,
    pub idle_splash_after_seconds: u64,
    pub locale: String,
    #[serde(default)]
    pub ca_cert_path: Option<PathBuf>,
}

impl Config {
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yml::from_str(&text).context("failed to parse configuration")
    }
}

pub(crate) fn validate_locale(locale: &str) -> Result<()> {
    let available = rust_i18n::available_locales!();

    if available
        .iter()
        .any(|available_locale| available_locale == locale)
    {
        return Ok(());
    }

    bail!(
        "unsupported TUI locale '{locale}'; supported locales are: {}",
        available.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ca_cert_path_option() {
        let yaml = r#"
tui:
  api_base_url: "https://localhost:3000"
  event_poll_interval_ms: 20
  scanner_max_gap_ms: 80
  idle_splash_after_seconds: 60
  locale: "en"
  ca_cert_path: "certs/rootCA.pem"
"#;
        let config: Config = serde_yml::from_str(yaml).unwrap();
        assert_eq!(
            config.tui.ca_cert_path,
            Some(PathBuf::from("certs/rootCA.pem"))
        );

        let yaml_no_ca = r#"
tui:
  api_base_url: "https://localhost:3000"
  event_poll_interval_ms: 20
  scanner_max_gap_ms: 80
  idle_splash_after_seconds: 60
  locale: "en"
"#;
        let config_no_ca: Config = serde_yml::from_str(yaml_no_ca).unwrap();
        assert_eq!(config_no_ca.tui.ca_cert_path, None);
    }
}
