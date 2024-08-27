use bytes::Bytes;
use reqwest::blocking::Client;
use reqwest::Result;
use std::env;

pub struct Service {
    client: Client,
    api_key: String,
}

impl Service {
    pub fn new() -> Service {
        Service {
            client: reqwest::blocking::Client::new(),
            api_key: env::var("API_KEY_DEV").unwrap(),
        }
    }

    pub fn get(&self, logo: String) -> Result<Bytes> {
        let url = format!("https://img.logo.dev/{}?token={}", logo, self.api_key);

        let request = self.client.get(url).send()?.error_for_status()?.bytes()?;

        Ok(request)
    }
}
