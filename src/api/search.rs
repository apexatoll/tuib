use super::*;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct SearchResult {
    title: String,

    #[serde(rename = "videoId")]
    video_id: String,
}

struct Search {
    query: String,
}

impl Search {
    fn new(query: String) -> Self {
        Self { query }
    }
}

impl Endpoint<SearchResult> for Search {
    const PATH: &'static str = "api/v1/search";

    fn params(&self) -> Vec<(&str, &str)> {
        vec![
            ("q", &self.query),
            ("type", "video"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod endpoint {
        use super::*;

        #[test]
        fn it_returns_the_expected_endpoint() {
            let query = String::from("learn rust");
            let instance = Instance::default();
            let actual = Search::new(query).endpoint(&instance).unwrap();

            let expected = String::from(
                "http://example.com/api/v1/search?q=learn+rust&type=video"
            );

            assert_eq!(actual, expected);
        }
    }

    mod fetch {
        use super::*;

        #[tokio::test]
        async fn it_fetches_and_parses_search_results() {
            let server = Server::run();
            let client = Client::new();
            let instance = Instance::from(&server);

            let query = String::from("learn rust");

            let response = json!([
                {
                    "title": "Video one",
                    "videoId": "abc",
                },
                {
                    "title": "Video two",
                    "videoId": "def",
                },
                {
                    "title": "Video three",
                    "videoId": "123",
                },
            ]);

            server.expect(
                Expectation::matching(all_of![
                    request::method_path("GET", "/api/v1/search"),
                    request::query(url_decoded(contains(("q", "learn rust")))),
                    request::query(url_decoded(contains(("type", "video")))),
                ]).respond_with(json_encoded(response))
            );

            let actual = Search::new(query)
                .fetch(&instance, &client)
                .await
                .unwrap();

            let expected = vec![
                SearchResult {
                    title: String::from("Video one"),
                    video_id: String::from("abc"),
                },
                SearchResult {
                    title: String::from("Video two"),
                    video_id: String::from("def"),
                },
                SearchResult {
                    title: String::from("Video three"),
                    video_id: String::from("123"),
                },
            ];

            assert_eq!(actual, expected);
        }
    }
}
