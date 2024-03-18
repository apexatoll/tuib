use super::*;
use crate::api::{Instance, SearchResult};
use reqwest::Client;

#[derive(Default)]
pub struct App {
    pub client: Client,
    pub instance: Instance,
    pub mode: Mode,
    pub query: String,
    pub results: Vec<SearchResult>,
    pub cursor: ListState,
    pub is_running: bool,
}

#[derive(Default)]
pub enum Mode {
    #[default] Search,
    Browse,
}

impl App {
    const SOURCE: &'static str = "https://api.invidious.io";

    pub async fn setup() -> Result<Self> {
        let client = Client::new();
        let instance = Instance::fetch(Self::SOURCE, &client).await?;

        Ok(Self {
            instance,
            client,
            ..Default::default()
        })
    }
}
