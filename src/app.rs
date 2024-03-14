use super::*;
use reqwest::Client;

#[derive(Default)]
pub struct App {
    client: Client,
}
