use marvel_api_rs::model::{
    Character, CharactersFilter, Comic, ComicsFilter, Creator, CreatorsFilter, Event, EventsFilter,
    Series, SeriesFilter, StoriesFilter, Story,
};

#[test]
fn get_all_characters_returns_200() {
    // Arrange
    let mut filter = CharactersFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Character::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}

#[test]
fn get_all_comics_returns_200() {
    // Arrange
    let mut filter = ComicsFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Comic::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}

#[test]
fn get_all_creators_returns_200() {
    // Arrange
    let mut filter = CreatorsFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Creator::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}

#[test]
fn get_all_events_returns_200() {
    // Arrange
    let mut filter = EventsFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Event::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}

#[test]
fn get_all_series_returns_200() {
    // Arrange
    let mut filter = SeriesFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Series::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}

#[test]
fn get_all_stories_returns_200() {
    // Arrange
    let mut filter = StoriesFilter::new();
    filter.limit = Some(10);

    // Act
    let response = Story::get_all(Some(filter)).unwrap();

    // Assert
    assert_eq!(200, response.code.unwrap());
}
