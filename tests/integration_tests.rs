
use std::time::Duration;

use marvel_api_rs::{api::Fault, model::Character};



#[test]
fn get_all_characters_returns_200() {
    // let mut res =surf::get("https://gateway.marvel.com/v1/public/characters?apikey=aa03547a3f992f9008cb460f61ce8b31&ts=1619535520001&hash=73a10fdf995f452487852e513efba4a9")
    // .await();

    let response = Character::get_all(None).unwrap();

    println!("{:?}", response);
    // println!("{:?}", res.text().unwrap());
    // match Character::get_all(None) {
    //     Ok(w) => {
    //         assert_eq!(w.code.unwrap(), 200);
    //     }
    //     Err(error) => {
    //         println!("{:?}", error);
    //         assert!(false);
    //     }
    // }

    assert!(false)
}
