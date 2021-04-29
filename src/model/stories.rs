use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};

use crate::{
    api::{ApiError, Filter, RequestHandler},
    model::{creators::CreatorsFilter, events::EventsFilter, series::SeriesFilter, NoneFilter},
};

use super::{
    characters::CharacterList,
    comics::{ComicList, ComicSummary, ComicsFilter},
    common::Image,
    creators::CreatorList,
    data_wrapper::DataWrapper,
    events::EventList,
    series::SeriesList,
    CharactersFilter,
};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct Story {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    pub r#type: Option<String>,
    pub modified: Option<DateTime<Utc>>,
    pub thumbnail: Option<Image>,
    pub comics: Option<ComicList>,
    pub series: Option<SeriesList>,
    pub events: Option<EventList>,
    pub characters: Option<CharacterList>,
    pub creators: Option<CreatorList>,
    pub original_issue: Option<ComicSummary>,
}

#[derive(Debug, Deserialize)]
pub struct StoryList {
    pub available: Option<u32>,
    pub returned: Option<u32>,
    #[serde(rename = "collectionURI")]
    pub collection_uri: Option<String>,
    #[serde(default)]
    pub items: Vec<StorySummary>,
}

#[derive(Debug, Deserialize)]
pub struct StorySummary {
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

pub struct StoriesFilter {
    pub modified_since: Option<DateTime<Utc>>,
    pub comics: Option<String>,
    pub series: Option<String>,
    pub events: Option<String>,
    pub creators: Option<String>,
    pub characters: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
impl Filter for StoriesFilter {
    fn build(self, url: String) -> String {
        let mut url = url::Url::parse(&url).unwrap();

        let mut queries = HashMap::new();

        self.insert(
            &mut queries,
            "modifiedSince".into(),
            self.modified_since.as_ref(),
        )
        .insert(&mut queries, "events".into(), self.events.as_ref())
        .insert(&mut queries, "series".into(), self.series.as_ref())
        .insert(&mut queries, "creators".into(), self.creators.as_ref())
        .insert(&mut queries, "characters".into(), self.characters.as_ref())
        .insert(&mut queries, "comics".into(), self.comics.as_ref())
        .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
        .insert(&mut queries, "limit".into(), self.limit.as_ref())
        .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl StoriesFilter {
    pub fn new() -> Self {
        StoriesFilter {
            modified_since: None,
            comics: None,
            series: None,
            events: None,
            creators: None,
            characters: None,
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    fn insert<T: Display>(
        &self,
        queries: &mut HashMap<String, String>,
        name: String,
        value: Option<&T>,
    ) -> &Self {
        if value.is_some() {
            queries.insert(name, value.unwrap().to_string());
        }
        self
    }
}

impl Story {
    pub fn get_all(filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>("v1/public/stories", filter)
    }

    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(&format!("v1/public/stories/{}", id), None)
    }

    pub fn get_characters(id: u32, filter: Option<CharactersFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CharactersFilter>(
            &format!("v1/public/stories/{}/characters", id),
            filter,
        )
    }

    pub fn get_comics(id: u32, filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>(
            &format!("v1/public/stories/{}/comics", id),
            filter,
        )
    }

    pub fn get_creators(id: u32, filter: Option<CreatorsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CreatorsFilter>(
            &format!("v1/public/stories/{}/creators", id),
            filter,
        )
    }

    pub fn get_events(id: u32, filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>(
            &format!("v1/public/stories/{}/events", id),
            filter,
        )
    }

    pub fn get_series(id: u32, filter: Option<SeriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, SeriesFilter>(
            &format!("v1/public/stories/{}/series", id),
            filter,
        )
    }
}
