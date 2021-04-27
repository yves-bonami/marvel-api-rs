use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Story {}

#[derive(Debug, Deserialize)]
pub struct StoryList {}

#[derive(Debug, Deserialize)]
pub struct StorySummary {}

pub struct StoriesFilter {}
impl Filter for StoriesFilter {
    fn build(self, url: String) -> String {
        todo!()
    }
}
