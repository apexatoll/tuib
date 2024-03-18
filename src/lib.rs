#![allow(unused)]

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub use app::App;
pub use crossterm::event::{self, *};
pub use ratatui::{prelude::*, widgets::*};

mod api;
mod app;
mod ui;
