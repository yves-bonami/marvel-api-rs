use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub path: Option<String>,
    pub extension: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TextObject {
    pub r#type: Option<String>,
    pub language: Option<String>,
    pub text: Option<String>,
}

pub struct NoneFilter {}
impl Filter for NoneFilter {
    fn build(self, url: String) -> String {
        url.to_string()
    }
}
