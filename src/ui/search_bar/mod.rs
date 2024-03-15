use super::*;

pub struct SearchBar;

pub enum Message {
    Append(char),
    Delete,
    None,
}
