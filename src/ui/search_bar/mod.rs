use super::*;

mod renderer;
mod router;

pub struct SearchBar;

pub enum Message {
    Append(char),
    Delete,
    None,
}
