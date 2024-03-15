use super::*;
use api::SearchResult;

pub struct Browser;

pub enum Message {
    Up,
    Down,
    Select,
    None,
}
