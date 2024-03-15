use super::*;
use api::SearchResult;

mod renderer;

pub struct Browser;

pub enum Message {
    Up,
    Down,
    Select,
    None,
}
