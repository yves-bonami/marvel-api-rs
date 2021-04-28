use marvel_api_rs::model::Character;

#[test]
fn get_all_characters_returns_200() {
    let response = Character::get_all(None).unwrap();
    assert_eq!(200, response.code.unwrap());
}
