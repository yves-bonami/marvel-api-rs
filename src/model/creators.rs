use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Creator {}

#[derive(Debug, Deserialize)]
pub struct CreatorList {}

#[derive(Debug, Deserialize)]
pub struct CreatorSummary {}

pub struct CreatorsFilter {}
impl Filter for CreatorsFilter {
    fn build(self, url: String) -> String {
        todo!()
    }
}
