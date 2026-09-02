use crate::api::{self, client::ApiClient};
use crate::app::{App, Message};
use crate::input::Input;
use crate::splash::Splash;
use crate::ui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use std::time::{Duration, Instant};

const STARTUP_SPLASH_DURATION: Duration = Duration::from_millis(1_250);

pub(crate) enum DisplayState {
    StartupSplash { started: Instant },
    Active,
    Idle,
}

pub(crate) struct Runtime {
    pub(crate) app: App,
    pub(crate) api: ApiClient,
    pub(crate) input: Input,
    pub(crate) event_poll_interval: Duration,
    pub(crate) idle_timeout: Duration,
    pub(crate) last_activity: Instant,
    pub(crate) splash: Splash,
    pub(crate) display_state: DisplayState,
}

impl Runtime {
    pub(crate) fn new(
        app: App,
        api: ApiClient,
        input: Input,
        event_poll_interval: Duration,
        idle_timeout: Duration,
        splash: Splash,
    ) -> Self {
        Self {
            app,
            api,
            input,
            event_poll_interval,
            idle_timeout,
            last_activity: Instant::now(),
            splash,
            display_state: DisplayState::StartupSplash {
                started: Instant::now(),
            },
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        self.update_display_state();

        match self.display_state {
            DisplayState::StartupSplash { .. } | DisplayState::Idle => {
                self.splash.draw(frame);
            }
            DisplayState::Active => {
                ui::draw(frame, &mut self.app);
            }
        }
    }

    fn update_display_state(&mut self) {
        match self.display_state {
            DisplayState::StartupSplash { started }
            if started.elapsed() >= STARTUP_SPLASH_DURATION =>
                {
                    self.activate()
                }

            DisplayState::Active if self.last_activity.elapsed() >= self.idle_timeout => {
                self.idle()
            }

            _ => {}
        }
    }

    fn activate(&mut self) {
        self.display_state = DisplayState::Active;
        self.last_activity = Instant::now();
    }

    fn idle(&mut self) {
        self.splash.begin_idle();
        self.display_state = DisplayState::Idle;
        self.app.dialogs.clear();
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        let messages = self.input.handle(&mut self.app, key);

        for message in messages {
            self.dispatch(message).await;
        }
    }

    async fn handle_global_key(&mut self, key: KeyEvent) -> bool {
        if key.kind != KeyEventKind::Press || !key.modifiers.contains(KeyModifiers::CONTROL) {
            return false;
        }

        match key.code {
            KeyCode::Char('q') => {
                self.dispatch(Message::Quit).await;
                true
            }

            KeyCode::Char('s') => match self.display_state {
                DisplayState::Idle => {
                    self.splash.next_variant();
                    true
                }

                DisplayState::Active { .. } => {
                    self.splash.begin_idle();
                    self.display_state = DisplayState::Idle;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub(crate) async fn handle_event(&mut self, event: Event) {
        self.update_display_state();

        match event {
            Event::Key(key) => {
                if self.handle_global_key(key).await {
                    return;
                }

                self.activate();
                self.handle_key(key).await;
            }

            _ => {}
        }
    }

    pub(crate) async fn dispatch(&mut self, message: Message) {
        let mut next = Some(message);

        while let Some(message) = next {
            next = match self.app.update(message) {
                Some(command) => Some(api::execute::execute_command(&self.api, command).await),

                None => None,
            };
        }
    }
}
