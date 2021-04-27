use std::any::{self, Any};

use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Image {}

#[derive(Debug, Deserialize)]
pub struct Url {}

pub struct NoneFilter {}
impl Filter for NoneFilter {
    fn build(self, url: String) -> String {
        url.to_string()
    }
}