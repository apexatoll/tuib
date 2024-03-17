use super::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value as JSON, json};
use url::Url;

#[cfg(test)]
use httptest::{Server, Expectation, matchers::*, responders::*};

mod instance;
pub use instance::Instance;

mod endpoint;
use endpoint::Endpoint;

mod search;
pub use search::{SearchResult, submit as search};
