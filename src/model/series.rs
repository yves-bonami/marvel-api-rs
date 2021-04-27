use chrono::{DateTime, Utc};

use crate::api::Filter;

use super::{
    characters::CharacterList,
    comics::ComicList,
    common::{Image, Url},
    creators::CreatorList,
    events::EventList,
    stories::StoryList,
};

#[derive(Debug, Deserialize)]
pub struct Series {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    #[serde(default)]
    pub urls: Vec<Url>,
    pub start_year: Option<u32>,
    pub end_year: Option<u32>,
    pub rating: Option<String>,
    pub modified: Option<DateTime<Utc>>,
    pub thumbnail: Option<Image>,
    pub comics: Option<ComicList>,
    pub stories: Option<StoryList>,
    pub events: Option<EventList>,
    pub characters: Option<CharacterList>,
    pub creators: Option<CreatorList>,
    pub next: Option<SeriesSummary>,
    pub previous: Option<SeriesSummary>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesList {
    pub available: Option<u32>,
    pub returned: Option<u32>,
    pub collection_uri: Option<u32>,
    #[serde(default)]
    pub items: Vec<SeriesSummary>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesSummary {
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    pub name: Option<String>,
}

pub struct SeriesFilter {}
impl Filter for SeriesFilter {
    fn build(self, url: String) -> String {
        todo!()
    }
}
