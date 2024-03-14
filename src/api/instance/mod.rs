use super::*;

mod fetcher;

pub struct Instance {
    url: Url,
}

impl From<&str> for Instance {
    fn from(value: &str) -> Self {
        let url = Url::parse(value).unwrap();

        Self { url }
    }
}
