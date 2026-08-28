use crate::app::Message;

#[derive(Debug)]
pub(crate) enum PageResult<T> {
    Consumed,
    Message(Message),
    Unhandled(T),
}
