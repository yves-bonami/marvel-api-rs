use crate::api::Filter;

#[derive(Debug, Deserialize)]
pub struct Event {}

#[derive(Debug, Deserialize)]
pub struct EventList {}

#[derive(Debug, Deserialize)]
pub struct EventSummary {}

pub struct EventsFilter {}
impl Filter for EventsFilter {
    fn build(self, url: String) -> String {
        todo!()
    }
}