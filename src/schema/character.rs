use super::common::{DataList, Image, Url};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub modified: String,
    #[serde(rename = "resourceURI")]
    pub resource_uri: String,
    pub urls: Vec<Url>,
    pub thumbnail: Image,
    pub comics: DataList,
    pub series: DataList,
    pub stories: DataList,
    pub events: DataList,
}
