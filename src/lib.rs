#![allow(unused)]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod api;
mod app;

use app::App;
use crossterm::event::{self, *};
use ratatui::{prelude::*, widgets::*};
