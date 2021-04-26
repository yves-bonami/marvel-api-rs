use super::data_container::DataContainer;

#[derive(Debug, Deserialize)]
pub struct DataWrapper{
    pub code: Option<u32>,
    pub status: Option<String>,
    pub copyright: Option<String>,
    pub attribution_text: Option<String>,
    pub attribution_html: Option<String>,
    pub data: Option<DataContainer>,
    pub etag: Option<String>,
}