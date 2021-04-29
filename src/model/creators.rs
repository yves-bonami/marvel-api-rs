use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};

use crate::{
    api::{ApiError, Filter, RequestHandler},
    model::{comics::ComicsFilter, series::SeriesFilter, NoneFilter},
};

use super::{
    comics::ComicList,
    common::{Image, Url},
    data_wrapper::DataWrapper,
    events::{EventList, EventsFilter},
    series::SeriesList,
    stories::{StoriesFilter, StoryList},
};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct Creator {
    pub id: Option<u32>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub suffix: Option<String>,
    pub full_name: Option<String>,
    pub modified: Option<DateTime<Utc>>,
    pub resource_uri: Option<String>,
    #[serde(default)]
    pub urls: Vec<Url>,
    pub thumbnail: Option<Image>,
    pub series: Option<SeriesList>,
    pub stories: Option<StoryList>,
    pub comics: Option<ComicList>,
    pub events: Option<EventList>,
}

#[derive(Debug, Deserialize)]
pub struct CreatorList {
    pub available: Option<u32>,
    pub returned: Option<u32>,
    pub collection_uri: Option<String>,
    #[serde(default)]
    pub items: Vec<CreatorSummary>,
}

#[derive(Debug, Deserialize)]
pub struct CreatorSummary {
    pub resource_uri: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
}

pub struct CreatorsFilter {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub suffix: Option<String>,
    pub name_starts_with: Option<String>,
    pub first_name_starts_with: Option<String>,
    pub middle_name_starts_with: Option<String>,
    pub last_name_starts_with: Option<String>,
    pub modified_since: Option<DateTime<Utc>>,
    pub comics: Option<String>,
    pub series: Option<String>,
    pub events: Option<String>,
    pub stories: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
impl Filter for CreatorsFilter {
    fn build(self, url: String) -> String {
        let mut url = url::Url::parse(&url).unwrap();

        let mut queries = HashMap::new();

        self.insert(&mut queries, "firstName".into(), self.first_name.as_ref())
            .insert(&mut queries, "middleName".into(), self.middle_name.as_ref())
            .insert(&mut queries, "lastName".into(), self.last_name.as_ref())
            .insert(&mut queries, "suffix".into(), self.suffix.as_ref())
            .insert(
                &mut queries,
                "nameStartsWith".into(),
                self.name_starts_with.as_ref(),
            )
            .insert(
                &mut queries,
                "firstNameStartsWith".into(),
                self.first_name_starts_with.as_ref(),
            )
            .insert(
                &mut queries,
                "middleNameStartsWith".into(),
                self.middle_name_starts_with.as_ref(),
            )
            .insert(
                &mut queries,
                "lastNameStartsWith".into(),
                self.last_name_starts_with.as_ref(),
            )
            .insert(
                &mut queries,
                "modifiedSince".into(),
                self.modified_since.as_ref(),
            )
            .insert(&mut queries, "comics".into(), self.comics.as_ref())
            .insert(&mut queries, "series".into(), self.series.as_ref())
            .insert(&mut queries, "events".into(), self.events.as_ref())
            .insert(&mut queries, "stories".into(), self.stories.as_ref())
            .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
            .insert(&mut queries, "limit".into(), self.limit.as_ref())
            .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl CreatorsFilter {
    pub fn new() -> Self {
        CreatorsFilter {
            first_name: None,
            middle_name: None,
            last_name: None,
            suffix: None,
            name_starts_with: None,
            first_name_starts_with: None,
            middle_name_starts_with: None,
            last_name_starts_with: None,
            modified_since: None,
            comics: None,
            series: None,
            events: None,
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

impl Creator {
    pub fn get_all(filter: Option<CreatorsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CreatorsFilter>("v1/public/creators", filter)
    }

    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(&format!("v1/public/creators/{}", id), None)
    }

    pub fn get_comics(id: u32, filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>(
            &format!("v1/public/creators/{}/comics", id),
            filter,
        )
    }

    pub fn get_events(id: u32, filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>(
            &format!("v1/public/creators/{}/events", id),
            filter,
        )
    }

    pub fn get_series(id: u32, filter: Option<SeriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, SeriesFilter>(
            &format!("v1/public/creators/{}/series", id),
            filter,
        )
    }

    pub fn get_stories(id: u32, filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>(
            &format!("v1/public/creators/{}/stories", id),
            filter,
        )
    }
}
