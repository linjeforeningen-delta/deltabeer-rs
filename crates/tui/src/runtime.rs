use crate::api;
use crate::api::client::ApiClient;
use crate::app::{App, Message};
use crate::input::Input;
use crossterm::event::KeyEvent;
use std::time::Duration;

pub(crate) struct Runtime {
    pub(crate) app: App,
    pub(crate) api: ApiClient,
    pub(crate) input: Input,
    pub(crate) event_poll_interval: Duration,
}

impl Runtime {
    pub(crate) fn new(
        app: App,
        api: ApiClient,
        input: Input,
        event_poll_interval: Duration,
    ) -> Self {
        Self {
            app,
            api,
            input,
            event_poll_interval,
        }
    }

    pub(crate) async fn handle_key(&mut self, key: KeyEvent) {
        let messages = self.input.handle(&mut self.app, key);

        for message in messages {
            self.dispatch(message).await;
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
