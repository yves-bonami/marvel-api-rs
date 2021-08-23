use crate::{common::Endpoint, schema::common::ResponseMessage, Result};
use serde::de::DeserializeOwned;
use tracing::debug;

#[cfg(test)]
use mockito;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    public_key: String,
    private_key: String,
}

impl Default for Client {
    fn default() -> Self {
        #[cfg(not(test))]
        let url = "https://gateway.marvel.com";
        #[cfg(test)]
        let url = &mockito::server_url();

        Self::new(url).expect("Error creating API client")
    }
}

impl Client {
    fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: base_url.into(),
            public_key: "".into(),
            private_key: "".into(),
        })
    }

    pub fn set_api_key(&mut self, public_key: &str, private_key: &str) -> Result<Self> {
        self.public_key = public_key.into();
        self.private_key = private_key.into();
        Ok(self.clone())
    }

    pub fn send_request<E>(
        &self,
        request: &dyn Endpoint,
    ) -> Result<ResponseMessage<E>, attohttpc::Error>
    where
        E: DeserializeOwned,
        E: Default,
    {
        let url = format!("{}/{}", self.base_url, request.path());
        let mut params = request.params();
        params.insert("apikey", self.public_key.to_string());
        params.insert("ts", "1".to_string());
        let to_hash = format!("{}{}{}", 1, self.private_key, self.public_key);
        params.insert("hash", format!("{:x}", md5::compute(to_hash)));

        debug!("{:?}", params);
        debug!("{:?}", url);

        let resp = attohttpc::RequestBuilder::new(request.method(), url)
            .params(params)
            .send()?
            .json::<ResponseMessage<E>>()?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        let _ = Client::default()
            .set_api_key("public key", "private key")
            .unwrap();
    }
}
