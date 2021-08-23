use marvel_api::{api::character, Client};
use std::env;
use tracing::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let client = Client::default().set_api_key(
        "aa03547a3f992f9008cb460f61ce8b31",
        "d3c3e867cae833b35a9f78108dc02f1cf7125efb",
    )?;

    let req = character::ListCharactersRequest {
        limit: Some(1),
        name_starts_with: Some("Moon".to_string()),
        ..character::ListCharactersRequest::default()
    };

    let res = req.send(&client)?;

    info!("{:#?}", res);

    Ok(())
}
