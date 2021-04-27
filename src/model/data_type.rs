use super::{series::Series, Character, Comic, Creator, Event, Story};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    Character(Character),
    Comic(Comic),
    Creator(Creator),
    Event(Event),
    Series(Series),
    Story(Story),
}
