use super::*;
use app::{App, Mode};

mod renderer;

pub struct Interface;

pub enum Message {
    Delegate(KeyEvent),
    Quit,
    None,
}
