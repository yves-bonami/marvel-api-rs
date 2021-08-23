#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseMessage<E>
where
    E: Default,
{
    Success(DataWrapper<E>),
    Error(ErrorMessage),
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct DataWrapper<E>
where
    E: Default,
{
    /// The HTTP status code of the returned result.
    pub code: u32,
    pub status: String,
    pub copyright: String,
    #[serde(rename = "attributionText")]
    pub attribution_text: String,
    #[serde(rename = "attributionHTML")]
    pub attribution_html: String,
    pub etag: String,
    pub data: DataContainer<E>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct DataContainer<E> {
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
    pub count: u32,
    pub results: E,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct DataList {
    pub available: u32,
    pub returned: u32,
    #[serde(rename = "collectionURI")]
    pub collection_uri: String,
    pub items: Vec<Summary>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Summary {
    #[serde(rename = "resourceURI")]
    pub resource_uri: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ErrorMessage {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Price {}

#[derive(Debug, Default, Deserialize)]
pub struct Date {}

#[derive(Debug, Default, Deserialize)]
pub struct TextObject {}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Url {
    /// A text identifier for the URL.
    pub r#type: String,
    /// A full URL (including scheme, domain, and path).
    pub url: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Image {
    /// The directory path of the image.
    pub path: String,
    /// The file extension for the image.
    pub extension: String,
}
