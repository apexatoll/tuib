use super::*;

pub trait Endpoint<T: for<'a> Deserialize<'a>> {
    const PATH: &'static str;

    fn params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn endpoint(&self, instance: &Instance) -> Result<String> {
        let url = instance.url.join(Self::PATH)?.to_string();

        if self.params().is_empty() {
            Ok(url)
        } else {
            let params = serde_urlencoded::to_string(self.params())?;

            Ok(format!("{url}?{params}"))
        }
    }

    async fn fetch(&self, instance: &Instance, client: &Client) -> Result<Vec<T>> {
        Ok(client
            .get(self.endpoint(instance)?)
            .send().await?
            .json::<Vec<T>>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod endpoint {
        use super::*;

        #[derive(Deserialize)]
        struct Item;

        mod without_params {
            use super::*;

            struct Resource;

            impl Endpoint<Item> for Resource {
                const PATH: &'static str = "test";
            }

            #[test]
            fn it_returns_the_expected_endpoint() {
                let instance = Instance::default();
                let actual = Resource.endpoint(&instance).unwrap();
                let expected = String::from("http://example.com/test");

                assert_eq!(actual, expected);
            }
        }

        mod with_params {
            use super::*;

            struct Resource;

            impl Endpoint<Item> for Resource {
                const PATH: &'static str = "test";

                fn params(&self) -> Vec<(&str, &str)> {
                    vec![
                        ("q", "hello world"),
                        ("type", "video"),
                    ]
                }
            }

            #[test]
            fn it_returns_the_expected_endpoint() {
                let instance = Instance::default();
                let actual = Resource.endpoint(&instance).unwrap();

                let expected = String::from(
                    "http://example.com/test?q=hello+world&type=video"
                );

                assert_eq!(actual, expected);
            }
        }
    }

    mod fetch {
        use super::*;

        #[derive(Deserialize, Debug, Eq, PartialEq)]
        struct Character {
            name: String,
            race: String,
        }

        struct Resource;

        impl Endpoint<Character> for Resource {
            const PATH: &'static str = "lotr";
        }

        #[tokio::test]
        async fn it_fetches_and_parses_items_from_endpoint() {
            let server = Server::run();
            let client = Client::new();
            let instance = Instance::from(&server);

            let response = json!([
                {
                    "name": "Frodo",
                    "race": "Hobbit",
                },
                {
                    "name": "Aragorn",
                    "race": "Man",
                },
                {
                    "name": "Gandalf",
                    "race": "Wizard",
                },
            ]);

            server.expect(
                Expectation::matching(
                    request::method_path("GET", "/lotr")
                ).respond_with(json_encoded(response))
            );

            let actual = Resource.fetch(&instance, &client).await.unwrap();

            let expected = vec![
                Character {
                    name: String::from("Frodo"),
                    race: String::from("Hobbit"),
                },
                Character {
                    name: String::from("Aragorn"),
                    race: String::from("Man"),
                },
                Character {
                    name: String::from("Gandalf"),
                    race: String::from("Wizard"),
                },
            ];

            assert_eq!(actual, expected);
        }
    }
}
