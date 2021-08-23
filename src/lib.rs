#[macro_use]
extern crate serde;

pub mod api;

mod client;
pub use client::Client;

mod common;

mod errors;
pub use errors::{ApiError, Result};

pub mod schema;
