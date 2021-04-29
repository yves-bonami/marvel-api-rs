use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};

use crate::{
    api::{ApiError, Filter, RequestHandler},
    model::{comics::ComicsFilter, events::EventsFilter, CharactersFilter, NoneFilter},
};

use super::{
    characters::CharacterList,
    comics::ComicList,
    common::{Image, Url},
    creators::{CreatorList, CreatorsFilter},
    data_wrapper::DataWrapper,
    events::EventList,
    stories::{StoriesFilter, StoryList},
};

type Result<T> = std::result::Result<T, ApiError>;

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

pub struct SeriesFilter {
    pub title: Option<String>,
    pub title_starts_with: Option<String>,
    pub start_year: Option<u32>,
    pub modified_since: Option<DateTime<Utc>>,
    pub comics: Option<String>,
    pub stories: Option<String>,
    pub events: Option<String>,
    pub creators: Option<String>,
    pub characters: Option<String>,
    pub series_type: Option<String>,
    pub contains: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
impl Filter for SeriesFilter {
    fn build(self, url: String) -> String {
        let mut url = url::Url::parse(&url).unwrap();

        let mut queries = HashMap::new();

        self.insert(&mut queries, "title".into(), self.title.as_ref())
            .insert(
                &mut queries,
                "titleStartsWith".into(),
                self.title_starts_with.as_ref(),
            )
            .insert(&mut queries, "startYear".into(), self.start_year.as_ref())
            .insert(
                &mut queries,
                "modifiedSince".into(),
                self.modified_since.as_ref(),
            )
            .insert(&mut queries, "events".into(), self.events.as_ref())
            .insert(&mut queries, "stories".into(), self.stories.as_ref())
            .insert(&mut queries, "creators".into(), self.creators.as_ref())
            .insert(&mut queries, "characters".into(), self.characters.as_ref())
            .insert(&mut queries, "comics".into(), self.comics.as_ref())
            .insert(&mut queries, "seriesType".into(), self.series_type.as_ref())
            .insert(&mut queries, "contains".into(), self.contains.as_ref())
            .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
            .insert(&mut queries, "limit".into(), self.limit.as_ref())
            .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl SeriesFilter {
    pub fn new() -> Self {
        SeriesFilter {
            title: None,
            title_starts_with: None,
            start_year: None,
            modified_since: None,
            comics: None,
            stories: None,
            events: None,
            creators: None,
            characters: None,
            series_type: None,
            contains: None,
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

impl Series {
    pub fn get_all(filter: Option<SeriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, SeriesFilter>("v1/public/series", filter)
    }

    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(&format!("v1/public/series/{}", id), None)
    }

    pub fn get_characters(id: u32, filter: Option<CharactersFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CharactersFilter>(
            &format!("v1/public/series/{}/characters", id),
            filter,
        )
    }

    pub fn get_comics(id: u32, filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>(
            &format!("v1/public/series/{}/comics", id),
            filter,
        )
    }

    pub fn get_creators(id: u32, filter: Option<CreatorsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CreatorsFilter>(
            &format!("v1/public/series/{}/creators", id),
            filter,
        )
    }

    pub fn get_events(id: u32, filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>(
            &format!("v1/public/series/{}/events", id),
            filter,
        )
    }

    pub fn get_stories(id: u32, filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>(
            &format!("v1/public/series/{}/stories", id),
            filter,
        )
    }
}
