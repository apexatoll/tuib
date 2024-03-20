use super::*;

impl Router<Message> for Browser {
    fn route(event: KeyEvent) -> Message {
        match event.code {
            KeyCode::Char('k') => Message::Up,
            KeyCode::Char('j') => Message::Down,
            KeyCode::Char('/') => Message::BackToSearch,
            KeyCode::Enter => Message::Select,
            _ => Message::None,
        }
    }
}
