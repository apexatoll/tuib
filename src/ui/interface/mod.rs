use super::*;
use app::{App, Mode};

mod renderer;
mod router;
mod handler;

pub struct Interface;

pub enum Message {
    Delegate(KeyEvent),
    Quit,
    None,
}
