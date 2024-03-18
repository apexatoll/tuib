use super::*;

impl Router<Message> for Interface {
    fn route(event: KeyEvent) -> Message {
        match event.modifiers {
            KeyModifiers::CONTROL => match event.code {
                KeyCode::Char('c') => Message::Quit,
                _ => Message::None,
            },

            KeyModifiers::NONE => Message::Delegate(event),

            _ => Message::None,
        }
    }
}
