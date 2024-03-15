use super::*;

mod renderer;
mod router;
mod handler;

pub struct SearchBar;

pub enum Message {
    Append(char),
    Delete,
    None,
}
