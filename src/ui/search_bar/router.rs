use super::*;

impl Router<Message> for SearchBar {
    fn route(event: KeyEvent) -> Message {
        match event.code {
            KeyCode::Char(char) => Message::Append(char),
            KeyCode::Backspace => Message::Delete,
            _ => Message::None,
        }
    }
}
