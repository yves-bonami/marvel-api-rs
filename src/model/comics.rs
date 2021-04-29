use std::{collections::HashMap, fmt::Display, u32};

use chrono::{DateTime, Utc};

use crate::{
    api::{ApiError, Filter, RequestHandler},
    model::{creators::CreatorsFilter, CharactersFilter, NoneFilter},
};

use super::{
    characters::CharacterList,
    common::{Image, TextObject, Url},
    creators::CreatorList,
    data_wrapper::DataWrapper,
    events::{EventList, EventsFilter},
    series::SeriesSummary,
    stories::{StoriesFilter, StoryList},
};

type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Deserialize)]
pub struct Comic {
    pub id: Option<u32>,
    pub digital_id: Option<u32>,
    pub title: Option<String>,
    pub issue_number: Option<u64>,
    pub variant_description: Option<String>,
    pub description: Option<String>,
    pub modified: Option<DateTime<Utc>>,
    pub isbn: Option<String>,
    pub upc: Option<String>,
    pub diamond_code: Option<String>,
    pub ean: Option<String>,
    pub issn: Option<String>,
    pub format: Option<String>,
    pub page_count: Option<u32>,
    #[serde(default)]
    pub text_objects: Vec<TextObject>,
    pub resource_uri: Option<String>,
    #[serde(default)]
    pub urls: Vec<Url>,
    pub series: Option<SeriesSummary>,
    #[serde(default)]
    pub variants: Vec<ComicSummary>,
    #[serde(default)]
    pub collections: Vec<ComicSummary>,
    #[serde(default)]
    pub collected_issues: Vec<ComicSummary>,
    #[serde(default)]
    pub dates: Vec<ComicDate>,
    #[serde(default)]
    pub prices: Vec<ComicPrice>,
    pub thumbnail: Option<Image>,
    #[serde(default)]
    pub images: Vec<Image>,
    pub creators: Option<CreatorList>,
    pub characters: Option<CharacterList>,
    pub stories: Option<StoryList>,
    pub events: Option<EventList>,
}

#[derive(Debug, Deserialize)]
pub struct ComicList {
    pub available: Option<u32>,
    pub returned: Option<u32>,
    pub collection_uri: Option<String>,
    #[serde(default)]
    pub items: Vec<ComicSummary>,
}

#[derive(Debug, Deserialize)]
pub struct ComicSummary {
    pub resource_uri: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ComicDate {
    pub r#type: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ComicPrice {
    pub r#type: Option<String>,
    pub price: Option<f32>,
}

pub struct ComicsFilter {
    pub format: Option<String>,
    pub format_type: Option<String>,
    pub no_variants: Option<bool>,
    pub date_descriptor: Option<String>,
    pub date_range: Option<String>,
    pub title: Option<String>,
    pub title_starts_with: Option<String>,
    pub start_year: Option<u32>,
    pub issue_number: Option<u32>,
    pub diamond_code: Option<String>,
    pub digital_id: Option<u32>,
    pub upc: Option<String>,
    pub isbn: Option<String>,
    pub ean: Option<String>,
    pub issn: Option<String>,
    pub has_digital_issue: Option<bool>,
    pub modified_since: Option<DateTime<Utc>>,
    pub creators: Option<String>,
    pub characters: Option<String>,
    pub series: Option<String>,
    pub events: Option<String>,
    pub stories: Option<String>,
    pub shared_appearances: Option<String>,
    pub collaborators: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
impl Filter for ComicsFilter {
    fn build(self, url: String) -> String {
        let mut url = url::Url::parse(&url).unwrap();

        let mut queries = HashMap::new();

        self.insert(&mut queries, "format".into(), self.format.as_ref())
            .insert(&mut queries, "formatType".into(), self.format_type.as_ref())
            .insert(&mut queries, "noVariants".into(), self.no_variants.as_ref())
            .insert(
                &mut queries,
                "dateDescriptor".into(),
                self.date_descriptor.as_ref(),
            )
            .insert(&mut queries, "dateRange".into(), self.date_range.as_ref())
            .insert(&mut queries, "title".into(), self.title.as_ref())
            .insert(
                &mut queries,
                "titleStartsWith".into(),
                self.title_starts_with.as_ref(),
            )
            .insert(&mut queries, "startYear".into(), self.start_year.as_ref())
            .insert(
                &mut queries,
                "issueNumber".into(),
                self.issue_number.as_ref(),
            )
            .insert(
                &mut queries,
                "diamondCode".into(),
                self.diamond_code.as_ref(),
            )
            .insert(&mut queries, "digitalId".into(), self.digital_id.as_ref())
            .insert(&mut queries, "upc".into(), self.upc.as_ref())
            .insert(&mut queries, "isbn".into(), self.isbn.as_ref())
            .insert(&mut queries, "ean".into(), self.ean.as_ref())
            .insert(&mut queries, "issn".into(), self.issn.as_ref())
            .insert(
                &mut queries,
                "hasDigitalIssue".into(),
                self.has_digital_issue.as_ref(),
            )
            .insert(
                &mut queries,
                "modifiedSince".into(),
                self.modified_since.as_ref(),
            )
            .insert(&mut queries, "creators".into(), self.creators.as_ref())
            .insert(&mut queries, "characters".into(), self.characters.as_ref())
            .insert(&mut queries, "series".into(), self.series.as_ref())
            .insert(&mut queries, "stories".into(), self.stories.as_ref())
            .insert(&mut queries, "events".into(), self.events.as_ref())
            .insert(
                &mut queries,
                "sharedAppearances".into(),
                self.shared_appearances.as_ref(),
            )
            .insert(
                &mut queries,
                "collaborators".into(),
                self.collaborators.as_ref(),
            )
            .insert(&mut queries, "orderBy".into(), self.order_by.as_ref())
            .insert(&mut queries, "limit".into(), self.limit.as_ref())
            .insert(&mut queries, "offset".into(), self.offset.as_ref());
        url.query_pairs_mut().extend_pairs(queries);
        url.into_string()
    }
}

impl ComicsFilter {
    pub fn new() -> Self {
        ComicsFilter {
            format: None,
            format_type: None,
            no_variants: None,
            date_descriptor: None,
            date_range: None,
            title: None,
            title_starts_with: None,
            start_year: None,
            issue_number: None,
            diamond_code: None,
            digital_id: None,
            upc: None,
            isbn: None,
            ean: None,
            issn: None,
            has_digital_issue: None,
            modified_since: None,
            creators: None,
            characters: None,
            series: None,
            events: None,
            stories: None,
            shared_appearances: None,
            collaborators: None,
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

impl Comic {
    pub fn get_all(filter: Option<ComicsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, ComicsFilter>("v1/public/comics", filter)
    }

    pub fn get(id: u32) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, NoneFilter>(&format!("v1/public/comics/{}", id), None)
    }

    pub fn get_characters(id: u32, filter: Option<CharactersFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CharactersFilter>(
            &format!("v1/public/comics/{}/characters", id),
            filter,
        )
    }

    pub fn get_creators(id: u32, filter: Option<CreatorsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, CreatorsFilter>(
            &format!("v1/public/comics/{}/creators", id),
            filter,
        )
    }

    pub fn get_events(id: u32, filter: Option<EventsFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, EventsFilter>(
            &format!("v1/public/comics/{}/events", id),
            filter,
        )
    }

    pub fn get_stories(id: u32, filter: Option<StoriesFilter>) -> Result<DataWrapper> {
        RequestHandler::get::<DataWrapper, StoriesFilter>(
            &format!("v1/public/comics/{}/stories", id),
            filter,
        )
    }
}
