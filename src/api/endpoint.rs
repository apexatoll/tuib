use super::*;

pub trait Endpoint {
    const PATH: &'static str;

    fn params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }
}
