mod app;
mod ui;
mod input;
pub(crate) mod auth;
pub mod api;
mod runtime;

use crate::api::client::ApiClient;
use crate::input::Input;
use anyhow::Result;
use app::App;
use crossterm::event::{self, Event};
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
        terminal.draw(|frame| ui::draw(frame, &runtime.app))?;

        if event::poll(Duration::from_millis(20))? {
            if let Event::Key(key) = event::read()? {
                runtime.handle_key(key).await;
            }
        }

        for message in runtime.input.tick(&mut runtime.app) {
            runtime.app.update(message);
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut runtime = Runtime::new(
        App::new(),
        ApiClient::new("http://localhost:3000"),
        Input::new(),
    );

    let result = run(&mut terminal, &mut runtime).await;

    restore_terminal(&mut terminal)?;
    result
}