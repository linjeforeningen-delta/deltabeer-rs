#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

pub mod api;
mod app;
pub(crate) mod auth;
mod config;
mod input;
mod runtime;
mod splash;
mod ui;

use crate::api::client::ApiClient;
use crate::input::Input;
use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::event;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use runtime::Runtime;
use std::io;
use std::time::Duration;

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
            runtime.app.update(message);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = config::Args::parse();
    let config = config::Config::load(&args.config)?;
    config::validate_locale(&config.tui.locale)?;
    rust_i18n::set_locale(&config.tui.locale);
    let mut terminal = init_terminal()?;
    let mut runtime = Runtime::new(
        App::new(),
        ApiClient::new(&config.tui.api_base_url),
        Input::new(Duration::from_millis(config.tui.scanner_max_gap_ms)),
        Duration::from_millis(config.tui.event_poll_interval_ms),
        Duration::from_secs(config.tui.idle_splash_after_seconds),
        splash::Splash::new()?,
    );

    let result = run(&mut terminal, &mut runtime).await;

    restore_terminal(&mut terminal)?;
    result
}
