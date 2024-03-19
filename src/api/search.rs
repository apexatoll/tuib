use super::*;

pub async fn submit(
    query: &String,
    instance: &Instance,
    client: &Client
) -> Result<Vec<SearchResult>> {
    Search::new(query.to_owned()).fetch(instance, client).await
}

#[cfg_attr(test, derive(Default))]
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
    pub author: String,

    #[serde(rename = "publishedText")]
    pub published: String,

    #[serde(rename = "lengthSeconds")]
    pub length: u32,

    #[serde(rename = "viewCount")]
    pub views: u64,

    #[serde(rename = "videoId")]
    pub video_id: String,
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
                    "description": "Lorem ipsum",
                    "author": "foo",
                    "publishedText": "1 month ago",
                    "lengthSeconds": 123,
                    "viewCount": 20,
                    "videoId": "abc",
                },
                {
                    "title": "Video two",
                    "description": "Dolor est",
                    "author": "bar",
                    "published": 12345,
                    "publishedText": "4 months ago",
                    "lengthSeconds": 123,
                    "viewCount": 200,
                    "videoId": "def",
                },
                {
                    "title": "Video three",
                    "description": "Quia amet",
                    "author": "baz",
                    "publishedText": "2 months ago",
                    "lengthSeconds": 123,
                    "viewCount": 2000,
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
                    description: String::from("Lorem ipsum"),
                    author: String::from("foo"),
                    published: String::from("1 month ago"),
                    length: 123,
                    views: 20,
                    video_id: String::from("abc"),
                },

                SearchResult {
                    title: String::from("Video two"),
                    description: String::from("Dolor est"),
                    author: String::from("bar"),
                    published: String::from("4 months ago"),
                    length: 123,
                    views: 200,
                    video_id: String::from("def"),
                },

                SearchResult {
                    title: String::from("Video three"),
                    description: String::from("Quia amet"),
                    author: String::from("baz"),
                    published: String::from("2 months ago"),
                    length: 123,
                    views: 2000,
                    video_id: String::from("123"),
                },
            ];

            assert_eq!(actual, expected);
        }
    }
}
