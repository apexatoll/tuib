use super::*;

pub trait Router<Message> {
    fn route(event: KeyEvent) -> Message;
}
