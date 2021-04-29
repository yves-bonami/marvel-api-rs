use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};

use crate::api::{ApiError, Filter, RequestHandler};

use super::{
    comics::{ComicList, ComicsFilter},
    common::{Image, NoneFilter, Url},
    data_wrapper::DataWrapper,
    events::{EventList, EventsFilter},
    series::{SeriesFilter, SeriesList},
    stories::{StoriesFilter, StoryList},
};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct Character {
    /// The unique ID of the character resource.
    pub id: Option<u32>,
    /// The name of the character.
    pub name: Option<String>,
    /// A short bio or description of the character.
    pub description: Option<String>,
    /// The date the resource was most recently modified.
    pub modified: Option<DateTime<Utc>>,
    /// The canonical URL identifier for this resource.
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    /// A set of public web site URLs for the resource.
    #[serde(default)]
    pub urls: Vec<Url>,
    /// The representative image for this character.
    pub thumbnail: Option<Image>,
    /// A resource list containing comics which feature this character.
    pub comics: Option<ComicList>,
    /// A resource list of stories in which this character appears.
    pub stories: Option<StoryList>,
    /// A resource list of events in which this character appears.
    pub events: Option<EventList>,
    /// A resource list of series in which this character appears.
    pub series: Option<SeriesList>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterList {
    /// The number of total available characters in this list. Will always be greater than or equal to the "returned" value.
    pub available: Option<u32>,
    /// The number of characters returned in this collection (up to 20).
    pub returned: Option<u32>,
    /// The path to the full list of characters in this collection.
    pub collection_uri: Option<String>,
    /// The list of returned characters in this collection.
    #[serde(default)]
    pub items: Vec<CharacterSummary>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterSummary {
    /// The path to the individual character resource.
    #[serde(rename = "resourceURI")]
    pub resource_uri: Option<String>,
    /// The full name of the character.
    pub name: Option<String>,
    /// The role of the creator in the parent entity.
    pub role: Option<String>,
}

pub struct CharactersFilter {
    /// Return only characters matching the specified full character name (e.g. Spider-Man).
    pub name: Option<String>,
    /// Return characters with names that begin with the specified string (e.g. Sp).
    pub name_starts_with: Option<String>,
    /// Return only characters which have been modified since the specified date.
    pub modified_since: Option<DateTime<Utc>>,
    /// Return only characters which appear in the specified comics (accepts a comma-separated list of ids).
    pub comics: Option<String>,
    /// Return only characters which appear the specified series (accepts a comma-separated list of ids).
    pub series: Option<String>,
    /// Return only characters which appear in the specified events (accepts a comma-separated list of ids).
    pub events: Option<String>,
    /// Return only characters which appear the specified stories (accepts a comma-separated list of ids).
    pub stories: Option<String>,
    /// Order the result set by a field or fields. Add a "-" to the value sort in descending order. Multiple values are given priority in the order in which they are passed
    pub order_by: Option<String>,
    /// Limit the result set to the specified number of resources.
    pub limit: Option<u32>,
    /// Skip the specified number of resources in the result set.
    pub offset: Option<u32>,
}

impl Filter for CharactersFilter {
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
            .insert(&mut queries, "comics".into(), self.comics.as_ref())
            .insert(&mut queries, "events".into(), self.events.as_ref())
            .insert(&mut queries, "stories".into(), self.stories.as_ref())
            .insert(&mut queries, "series".into(), self.series.as_ref())
            .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
            .insert(&mut queries, "limit".into(), self.limit.as_ref())
            .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl CharactersFilter {
    pub fn new() -> Self {
        CharactersFilter {
            name: None,
            name_starts_with: None,
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

impl Character {
    /// Fetches lists of comic characters with optional filters.
    pub fn get_all(filter: Option<CharactersFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CharactersFilter>("v1/public/characters", filter)
    }

    /// This method fetches a single character resource. It is the canonical URI for any character resource provided by the API.
    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(
            &format!("v1/public/characters/{}", id),
            None,
        )
    }

    /// Fetches lists of comics containing a specific character, with optional filters.
    pub fn get_comics(id: u32, filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>(
            &format!("v1/public/characters/{}/comics", id),
            filter,
        )
    }

    /// Fetches lists of events in which a specific character appears, with optional filters.
    pub fn get_events(id: u32, filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>(
            &format!("v1/public/characters/{}/events", id),
            filter,
        )
    }

    /// Fetches lists of comic series in which a specific character appears, with optional filters.
    pub fn get_series(id: u32, filter: Option<SeriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, SeriesFilter>(
            &format!("v1/public/characters/{}/series", id),
            filter,
        )
    }

    /// Fetches lists of comic stories featuring a specific character with optional filters.
    pub fn get_stories(id: u32, filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>(
            &format!("v1/public/characters/{}/stories", id),
            filter,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use mockito::{mock, Matcher};

    fn setup() -> () {
        env::set_var("MARVEL_PUBLIC", "12345");
        env::set_var("MARVEL_PRIVATE", "12345");
    }

    #[test]
    fn get_all_characters_returns_datawrapper() {
        setup();
        let mock = mock("GET", "/v1/public/characters")
            .with_status(200)
            .with_body(r#"{"code": 200, "status": "ok"}"#)
            .match_query(Matcher::Any)
            .create();

        let response = Character::get_all(None);
        mock.assert();
        match response {
            Ok(w) => {
                assert_eq!(w.code.unwrap(), 200);
                assert_eq!(w.status.unwrap(), "ok");
            }
            Err(_) => {
                assert!(false)
            }
        };
    }
}
