use super::*;
use app::{App, Mode};

pub struct Interface;

pub enum Message {
    Delegate(KeyEvent),
    Quit,
    None,
}
