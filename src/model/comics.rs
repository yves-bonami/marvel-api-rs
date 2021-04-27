use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Comic {}

#[derive(Debug, Deserialize)]
pub struct ComicList {}

pub struct ComicsFilter {}
impl Filter for ComicsFilter {
    fn build(self, url: String) -> String {
        todo!()
    }
}
