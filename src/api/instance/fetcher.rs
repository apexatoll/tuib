use super::*;

struct Fetcher {
    source: Url,
}

impl From<&str> for Fetcher {
    fn from(value: &str) -> Self {
        let source = Url::parse(value).unwrap();

        Self { source }
    }
}

impl Fetcher {
    const PATH: &'static str = "instances.json";

    fn endpoint(&self) -> Result<Url> { 
        Ok(self.source.join(Self::PATH)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod endpoint {
        use super::*;

        #[test]
        fn it_builds_the_endpoint() {
            let fetcher = Fetcher::from("http://example.com");

            let expected = Url::parse(
                "http://example.com/instances.json"
            ).unwrap();

            let actual = fetcher.endpoint().unwrap();

            assert_eq!(actual, expected);
        }
    }
}
