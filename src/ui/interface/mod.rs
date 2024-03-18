use super::*;
use app::{App, Mode};

mod renderer;
mod router;

pub struct Interface;

pub enum Message {
    Delegate(KeyEvent),
    Quit,
    None,
}
