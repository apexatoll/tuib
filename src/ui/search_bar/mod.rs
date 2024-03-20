use super::*;

mod renderer;
mod router;
mod handler;

pub struct SearchBar;

impl SearchBar {
    const MARGIN: u16 = 3;
}

pub enum Message {
    Append(char),
    Delete,
    Submit,
    None,
}
