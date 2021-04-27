use std::env;

use chrono::Utc;
use isahc::ReadResponseExt;
use serde::Deserialize;

use super::ApiError;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const BASE_URL: &str = "https://gateway.marvel.com";
pub trait Filter {
    fn build(self, url: String) -> String;
}

pub struct RequestHandler {}

impl RequestHandler {
    fn url(url: &str) -> String {
        #[cfg(not(test))]
        return format!("{}/{}", BASE_URL, url);
        #[cfg(test)]
        return format!("{}/{}", mockito::server_url(), url);
    }

    pub fn get<T: for<'de> Deserialize<'de>, F: Filter>(
        address: &str,
        filter: Option<F>,
    ) -> Result<T, ApiError> {
        let url = Self::url(address);

        let ts = Utc::now().timestamp_millis();
        let public_key = env::var("MARVEL_PUBLIC")
            .expect("Please set the ´MARVEL_PUBLIC´ environment variable with your public key.");
        let private_key = env::var("MARVEL_PRIVATE")
            .expect("Please set the ´MARVEL_PRIVATE´ environment variable with your private key.");
        let hash = build_hash(ts, &public_key, &private_key);

        let mut request_url = url::Url::parse(&match filter {
            Some(f) => f.build(url),
            None => url,
        })
        .expect("Could not parse URL");

        request_url
            .query_pairs_mut()
            .append_pair("apikey", &public_key)
            .append_pair("ts", &ts.to_string())
            .append_pair("hash", &hash);

        println!("{:?}", &request_url);

        let response = isahc::get(request_url.as_str());

        println!("{:?}", response);

        let result = match response {
            Ok(mut r) => {
                r.json::<T>().expect("Could not parse JSON")
            }
            Err(e) => {
                return Err(ApiError::Transport(e));
            }
        };
        Ok(result)
    }
}

fn build_hash(ts: i64, public_key: &str, private_key: &str) -> String {
    let to_hash = format!("{}{}{}", ts, private_key, public_key);
    format!("{:x}", md5::compute(to_hash))
}
