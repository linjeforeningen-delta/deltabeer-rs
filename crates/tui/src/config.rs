use anyhow::{Context, Result};
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
}

impl Config {
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_yml::from_str(&text).context("failed to parse configuration")
    }
}
