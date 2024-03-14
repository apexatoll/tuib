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

    async fn fetch_all(&self, client: &Client) -> Result<Vec<Instance>> {
        Ok(client
            .get(self.endpoint()?)
            .send().await?
            .json::<Vec<JSON>>().await?
            .into_iter()
            .map(|json| serde_json::from_str(&json[1].to_string()).unwrap())
            .filter(|result: &FetcherResult| result.api.unwrap_or(false))
            .map(Instance::from)
            .collect())
    }

    async fn fetch(&self, client: &Client) -> Result<Instance> {
        Ok(self.fetch_all(client).await?.into_iter().next().unwrap())
    }
}

#[derive(Deserialize)]
struct FetcherResult {
    api: Option<bool>,
    uri: String,
}

impl From<FetcherResult> for Instance {
    fn from(result: FetcherResult) -> Self {
        let url = Url::parse(&result.uri).unwrap();

        Self { url }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<&Server> for Fetcher {
        fn from(server: &Server) -> Self {
            let source = Url::parse(&server.url_str("/")).unwrap();

            Self { source }
        }
    }

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

    mod fetch_all {
        use super::*;

        #[tokio::test]
        async fn when_no_valid_results_returned() {
            let server = Server::run();
            let client = Client::new();
            let fetcher = Fetcher::from(&server);

            let response = json!([
                ["http://foo.io", { "api": false, "uri": "http://foo.io" }],
                ["http://bar.io", { "api": false, "uri": "http://bar.io" }],
                ["http://baz.io", { "api": null, "uri": "http://baz.io" }],
                ["http://boo.io", { "api": null, "uri": "http://boo.io" }],
                ["http://far.io", { "api": null, "uri": "http://far.io" }],
                ["http://faz.io", { "api": false, "uri": "http://faz.io" }],
            ]);

            server.expect(
                Expectation::matching(
                    request::method_path("GET", "/instances.json")
                ).respond_with(json_encoded(response))
            );

            let actual = fetcher.fetch_all(&client).await.unwrap();

            let expected = Vec::new();

            assert_eq!(actual, expected);
        }

        #[tokio::test]
        async fn when_valid_results_returned() {
            let server = Server::run();
            let client = Client::new();
            let fetcher = Fetcher::from(&server);

            let response = json!([
                ["http://foo.io", { "api": true, "uri": "http://foo.io" }],
                ["http://bar.io", { "api": false, "uri": "http://bar.io" }],
                ["http://baz.io", { "api": true, "uri": "http://baz.io" }],
                ["http://boo.io", { "api": null, "uri": "http://boo.io" }],
                ["http://far.io", { "api": null, "uri": "http://far.io" }],
                ["http://faz.io", { "api": false, "uri": "http://faz.io" }],
            ]);

            server.expect(
                Expectation::matching(
                    request::method_path("GET", "/instances.json")
                ).respond_with(json_encoded(response))
            );

            let actual = fetcher.fetch_all(&client).await.unwrap();

            let expected = vec![
                Instance::from("http://foo.io"),
                Instance::from("http://baz.io"),
            ];

            assert_eq!(actual, expected);
        }
    }

    mod fetch {
        use super::*;

        #[tokio::test]
        #[should_panic]
        async fn when_no_valid_results_returned() {
            let server = Server::run();
            let client = Client::new();
            let fetcher = Fetcher::from(&server);

            let response = json!([
                ["http://foo.io", { "api": false, "uri": "http://foo.io" }],
                ["http://bar.io", { "api": false, "uri": "http://bar.io" }],
                ["http://baz.io", { "api": null, "uri": "http://baz.io" }],
                ["http://boo.io", { "api": null, "uri": "http://boo.io" }],
                ["http://far.io", { "api": null, "uri": "http://far.io" }],
                ["http://faz.io", { "api": false, "uri": "http://faz.io" }],
            ]);

            server.expect(
                Expectation::matching(
                    request::method_path("GET", "/instances.json")
                ).respond_with(json_encoded(response))
            );

            fetcher.fetch(&client).await.unwrap();
        }

        #[tokio::test]
        async fn when_valid_results_returned() {
            let server = Server::run();
            let client = Client::new();
            let fetcher = Fetcher::from(&server);

            let response = json!([
                ["http://foo.io", { "api": false, "uri": "http://foo.io" }],
                ["http://bar.io", { "api": false, "uri": "http://bar.io" }],
                ["http://baz.io", { "api": true, "uri": "http://baz.io" }],
                ["http://boo.io", { "api": null, "uri": "http://boo.io" }],
                ["http://far.io", { "api": null, "uri": "http://far.io" }],
                ["http://faz.io", { "api": false, "uri": "http://faz.io" }],
            ]);

            server.expect(
                Expectation::matching(
                    request::method_path("GET", "/instances.json")
                ).respond_with(json_encoded(response))
            );

            let actual = fetcher.fetch(&client).await.unwrap();

            let expected = Instance::from("http://baz.io");

            assert_eq!(actual, expected);
        }
    }
}
