use std::collections::HashMap;

use attohttpc::Method;
use chrono::Local;

use crate::common::Endpoint;
use crate::schema::character::Character;
use crate::schema::common::ResponseMessage;
use crate::{ApiError, Client, Result};

/// Fetches lists of comic characters with optional filters.
#[derive(Debug, Default, Serialize)]
pub struct ListCharactersRequest {
    /// Return only characters matching the specified full character name (e.g. Spider-Man).
    pub name: Option<String>,
    /// Return characters with names that begin with the specified string (e.g. Sp).
    pub name_starts_with: Option<String>,
    /// Return only characters which have been modified since the specified date.
    pub modified_since: Option<chrono::DateTime<Local>>,
    /// Return only characters which appear in the specified comics
    /// (accepts a comma-separated list of ids).
    pub comics: Option<String>,
    /// Return only characters which appear the specified series
    /// (accepts a comma-separated list of ids).
    pub series: Option<String>,
    /// Return only characters which appear in the specified events
    /// (accepts a comma-separated list of ids).
    pub events: Option<String>,
    /// Return only characters which appear the specified stories
    /// (accepts a comma-separated list of ids).
    pub stories: Option<String>,
    /// Order the result set by a field or fields.
    /// Add a "-" to the value sort in descending order.
    /// Multiple values are given priority in the order in which they are passed.
    ///
    /// Allowed values:
    /// - name
    /// - modified
    pub order_by: Option<String>,
    /// Limit the result set to the specified number of resources.
    pub limit: Option<u32>,
    /// Skip the specified number of resources in the result set.
    pub offset: Option<u32>,
}

/// This method fetches a single character resource.
/// It is the canonical URI for any character resource provided by the API.
pub struct GetCharacterRequest {
    /// A single character id.
    pub id: u32,
}

impl ListCharactersRequest {
    pub fn send(&self, client: &Client) -> Result<ResponseMessage<Vec<Character>>> {
        let result = match client.send_request::<Vec<Character>>(self) {
            Ok(resp) => resp,
            Err(err) => match err {
                _ => {
                    return Err(ApiError::HttpError(err));
                }
            },
        };
        Ok(result)
    }
}

impl Endpoint for ListCharactersRequest {
    fn path(&self) -> String {
        String::from("/v1/public/characters")
    }

    fn method(&self) -> attohttpc::Method {
        Method::GET
    }

    fn params(&self) -> HashMap<&str, String> {
        let mut params = HashMap::<&str, String>::new();

        self.name
            .as_ref()
            .map(|v| params.insert("name", v.to_string()));
        self.name_starts_with
            .as_ref()
            .map(|v| params.insert("nameStartsWith", v.to_string()));
        self.modified_since
            .as_ref()
            .map(|v| params.insert("modifiedSince", v.to_string()));
        self.comics
            .as_ref()
            .map(|v| params.insert("comics", v.to_string()));
        self.series
            .as_ref()
            .map(|v| params.insert("series", v.to_string()));
        self.events
            .as_ref()
            .map(|v| params.insert("events", v.to_string()));
        self.stories
            .as_ref()
            .map(|v| params.insert("stories", v.to_string()));
        self.order_by
            .as_ref()
            .map(|v| params.insert("orderBy", v.to_string()));
        self.limit
            .as_ref()
            .map(|v| params.insert("limit", v.to_string()));
        self.offset
            .as_ref()
            .map(|v| params.insert("offset", v.to_string()));
        params
    }
}

impl GetCharacterRequest {}

impl Endpoint for GetCharacterRequest {
    fn path(&self) -> String {
        format!("/v1/public/characters/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn params(&self) -> HashMap<&str, String> {
        HashMap::<&str, String>::new()
    }
}
