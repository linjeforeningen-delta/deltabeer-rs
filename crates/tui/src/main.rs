//! DeltaBeer terminal user interface binary.
//!
//! The TUI owns presentation, input, and local application state and
//! communicates with the server through an HTTP client and shared API DTOs.
//! It has no direct persistence access; database ownership remains with the
//! server and its storage adapter.

#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

mod api;
mod app;
pub(crate) mod auth;
mod config;
mod input;
mod model;
mod runtime;
mod splash;
mod ui;

use crate::{api::client::ApiClient, input::Input};
use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use runtime::Runtime;
use std::io;
use std::time::Duration;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

const TUI_LOG_DIRECTORY: &str = "logs";
const TUI_LOG_FILE: &str = "tui.log";

fn init_tracing() -> Result<tracing_appender::non_blocking::WorkerGuard> {
    std::fs::create_dir_all(TUI_LOG_DIRECTORY)?;
    let appender = tracing_appender::rolling::daily(TUI_LOG_DIRECTORY, TUI_LOG_FILE);
    let (non_blocking, guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .try_init()
        .ok();

    Ok(guard)
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    runtime: &mut Runtime,
) -> Result<()> {
    while !runtime.app.should_quit {
        terminal.draw(|frame| runtime.draw(frame))?;

        if event::poll(runtime.event_poll_interval)? {
            runtime.handle_event(event::read()?).await;
        }

        for message in runtime.input.tick(&mut runtime.app) {
            runtime.dispatch(message).await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _tracing_guard = init_tracing()?;

    let args = config::Args::parse();
    let config = match config::Config::load(&args.config) {
        Ok(config) => config,
        Err(error) => {
            tracing::error!(error = %error, path = %args.config.display(), "failed to load configuration");
            return Err(error);
        }
    };
    if let Err(error) = config::validate_locale(&config.tui.locale) {
        tracing::error!(error = %error, locale = %config.tui.locale, "invalid configured locale");
        return Err(error);
    }
    rust_i18n::set_locale(&config.tui.locale);
    let api_client =
        match ApiClient::new(&config.tui.api_base_url, config.tui.ca_cert_path.as_deref()) {
            Ok(client) => client,
            Err(error) => {
                tracing::error!(error = %error, "failed to initialize API client");
                return Err(error);
            }
        };
    let mut terminal = match init_terminal() {
        Ok(terminal) => terminal,
        Err(error) => {
            tracing::error!(error = %error, "failed to initialize terminal");
            return Err(error);
        }
    };
    let splash = match splash::Splash::new() {
        Ok(splash) => splash,
        Err(error) => {
            tracing::error!(error = %error, "failed to initialize splash screen");
            return Err(error);
        }
    };
    let mut runtime = Runtime::new(
        App::new(),
        api_client,
        Input::new(Duration::from_millis(config.tui.scanner_max_gap_ms)),
        Duration::from_millis(config.tui.event_poll_interval_ms),
        Duration::from_secs(config.tui.idle_splash_after_seconds),
        splash,
    );

    tracing::info!("TUI started");
    let result = run(&mut terminal, &mut runtime).await;
    if let Err(error) = &result {
        tracing::error!(error = %error, "TUI stopped with an application error");
    }

    if let Err(error) = restore_terminal(&mut terminal) {
        tracing::error!(error = %error, "failed to restore terminal");
        return Err(error);
    }
    result
}
