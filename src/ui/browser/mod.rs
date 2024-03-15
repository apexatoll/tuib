use super::*;
use api::SearchResult;

mod renderer;
mod router;

pub struct Browser;

pub enum Message {
    Up,
    Down,
    Select,
    None,
}
