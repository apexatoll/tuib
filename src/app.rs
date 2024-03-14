use super::*;
use crate::api::Instance;
use reqwest::Client;

#[derive(Default)]
pub struct App {
    client: Client,
    instance: Instance,
}
