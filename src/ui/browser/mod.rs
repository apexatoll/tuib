use super::*;
use api::SearchResult;

mod renderer;
mod router;
mod handler;

pub struct Browser;

pub enum Message {
    Up,
    Down,
    Select,
    None,
}
