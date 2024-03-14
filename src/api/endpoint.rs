use super::*;

pub trait Endpoint {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod endpoint {
        use super::*;

        mod without_params {
            use super::*;

            struct Resource;

            impl Endpoint for Resource {
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

            impl Endpoint for Resource {
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
}
