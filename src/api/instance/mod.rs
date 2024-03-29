use super::*;

mod fetcher;

#[derive(Debug, Eq, PartialEq)]
pub struct Instance {
    pub url: Url,
}

impl From<&str> for Instance {
    fn from(value: &str) -> Self {
        let url = Url::parse(value).unwrap();

        Self { url }
    }
}

#[cfg(test)]
impl From<&Server> for Instance {
    fn from(server: &Server) -> Self {
        let url = Url::parse(&server.url_str("/")).unwrap();

        Self { url }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::from("http://example.com")
    }
}

impl Instance {
    pub async fn fetch(source: &str, client: &Client) -> Result<Self> {
        fetcher::fetch(source, client).await
    }
}
