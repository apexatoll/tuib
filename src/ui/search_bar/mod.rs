use super::*;

mod renderer;

pub struct SearchBar;

pub enum Message {
    Append(char),
    Delete,
    None,
}
