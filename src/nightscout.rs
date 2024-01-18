use crate::models::EntriesResponse;
use reqwest::StatusCode;

pub struct Nightscout<'a> {
    base_url: &'a str,
    client: reqwest::Client,
}

impl<'a> Nightscout<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Nightscout {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_entries(&self) -> Option<Vec<EntriesResponse>> {
        let response = self
            .client
            .get(format!("{0}/api/v1/entries.json", self.base_url))
            .send()
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            return None;
        }

        let encoded_response = response.json::<Vec<EntriesResponse>>().await;

        match encoded_response {
            Ok(json_response) => Some(json_response),
            Err(_) => None,
        }
    }
}
