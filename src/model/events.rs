use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};

use crate::{api::{ApiError, Filter, RequestHandler}, model::{CharactersFilter, NoneFilter, creators::CreatorsFilter, series::SeriesFilter}};

use super::{
    characters::CharacterList,
    comics::{ComicList, ComicsFilter},
    common::{Image, Url},
    creators::CreatorList,
    data_wrapper::DataWrapper,
    series::SeriesList,
    stories::{StoriesFilter, StoryList},
};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub resource_uri: Option<String>,
    #[serde(default)]
    pub urls: Vec<Url>,
    pub modified: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub thumbnail: Option<Image>,
    pub comics: Option<ComicList>,
    pub stories: Option<StoryList>,
    pub series: Option<SeriesList>,
    pub characters: Option<CharacterList>,
    pub creators: Option<CreatorList>,
    pub next: Option<EventSummary>,
    pub previous: Option<EventSummary>,
}

#[derive(Debug, Deserialize)]
pub struct EventList {
    pub available: Option<u32>,
    pub returned: Option<u32>,
    pub collection_uri: Option<String>,
    #[serde(default)]
    pub items: Vec<EventSummary>,
}

#[derive(Debug, Deserialize)]
pub struct EventSummary {
    pub resource_uri: Option<String>,
    pub name: Option<String>,
}

pub struct EventsFilter {
    pub name: Option<String>,
    pub name_starts_with: Option<String>,
    pub modified_since: Option<DateTime<Utc>>,
    pub creators: Option<String>,
    pub characters: Option<String>,
    pub series: Option<String>,
    pub comics: Option<String>,
    pub stories: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
impl Filter for EventsFilter {
    fn build(self, url: String) -> String {
        let mut url = url::Url::parse(&url).unwrap();

        let mut queries = HashMap::new();

        self.insert(&mut queries, "name".into(), self.name.as_ref())
            .insert(
                &mut queries,
                "nameStartsWith".into(),
                self.name_starts_with.as_ref(),
            )
            .insert(
                &mut queries,
                "modifiedSince".into(),
                self.modified_since.as_ref(),
            )
            .insert(&mut queries, "creators".into(), self.creators.as_ref())
            .insert(&mut queries, "characters".into(), self.characters.as_ref())
            .insert(&mut queries, "comics".into(), self.comics.as_ref())
            .insert(&mut queries, "series".into(), self.series.as_ref())
            .insert(&mut queries, "stories".into(), self.stories.as_ref())
            .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
            .insert(&mut queries, "limit".into(), self.limit.as_ref())
            .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl EventsFilter {
    pub fn new() -> Self {
        EventsFilter {
            name: None,
            name_starts_with: None,
            modified_since: None,
            creators: None,
            characters: None,
            series: None,
            comics: None,
            stories: None,
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

impl Event {
    pub fn get_all(filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>("v1/public/events", filter)
    }

    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(&format!("v1/public/events/{}", id), None)
    }

    pub fn get_characters(id: u32, filter: Option<CharactersFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CharactersFilter>(
            &format!("v1/public/events/{}/characters", id),
            filter,
        )
    }

    pub fn get_comics(id: u32, filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>(
            &format!("v1/public/events/{}/comics", id),
            filter,
        )
    }

    pub fn get_creators(id: u32, filter: Option<CreatorsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CreatorsFilter>(
            &format!("v1/public/events/{}/creators", id),
            filter,
        )
    }

    pub fn get_series(id: u32, filter: Option<SeriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, SeriesFilter>(
            &format!("v1/public/events/{}/series", id),
            filter,
        )
    }

    pub fn get_stories(id: u32, filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>(
            &format!("v1/public/events/{}/stories", id),
            filter,
        )
    }
}
