use super::*;
use crate::api::Instance;
use reqwest::Client;

#[derive(Default)]
pub struct App {
    client: Client,
    instance: Instance,
}

impl App {
    const SOURCE: &'static str = "https://api.invidious.io";

    pub async fn setup() -> Result<Self> {
        let client = Client::new();
        let instance = Instance::fetch(Self::SOURCE, &client).await?;

        Ok(Self {
            instance,
            client,
        })
    }
}
