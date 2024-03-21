use super::*;

mod renderer;
mod router;
mod handler;

pub struct SearchBar;

impl SearchBar {
    const MARGIN: u16 = 3;
    const INPUT_LINE: u16 = 6;

    pub fn cursor_position(&self, app: &App, frame: &Frame) -> (u16, u16) {
        let column = 
            if app.query.len() as u16 >= frame.size().width - Self::MARGIN - 1 {
                frame.size().width - Self::MARGIN
            } else {
                app.query.len() as u16 + 1
            };

        (column, Self::INPUT_LINE)
    }
}

pub enum Message {
    Append(char),
    Delete,
    Submit,
    None,
}
