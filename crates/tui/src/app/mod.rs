pub(crate) mod dialog;
pub(crate) mod fields;
pub(crate) mod message;
pub(crate) mod page;
mod state;
mod update;

pub(crate) use dialog::{Dialog, DialogOpenMode};
pub(crate) use fields::input::TextInput;
pub(crate) use message::{AppError, Message};
pub(crate) use page::{Page, PageId};
pub(crate) use state::App;
