#![allow(unused)]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod app;

use app::App;
