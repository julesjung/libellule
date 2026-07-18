use serde::Serialize;
use serde_json::json;
use url::Url;

use crate::{api::function::Function, crypto::encode_request_count};

#[derive(Debug)]
pub struct Disconnected;
#[derive(Debug)]
pub struct Connected;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct Session<S = Disconnected> {
    session_id: u32,
    request_count: u32,
    key: [u8; 16],
    iv: [u8; 16],
    state: S,
}

impl Session {
    pub fn new(session_id: u32) -> Session {
        Session {
            session_id,
            request_count: 0,
            key: *md5::compute(&[]),
            iv: [0u8; 16],
            state: Disconnected,
        }
    }

    pub fn encode_request_count(&self) -> String {
        encode_request_count(self.request_count, &self.key, &self.iv)
    }

    pub async fn call<'a, S>(
        &mut self,
        context: FunctionContext<'a>,
        data: S,
    ) -> Result<String, Error>
    where
        S: Serialize,
    {
        self.request_count += 1;
        let encoded_request_count = self.encode_request_count();

        let endpoint = format!(
            "appelfonction/3/{}/{}",
            self.session_id, encoded_request_count
        );

        let url = context.instance_url.join(&endpoint).unwrap();

        let body = json!({
            "id": "FonctionParametres",
            "no": encoded_request_count,
            "session": self.session_id,
            "dataSec": data
        });

        let response = context
            .http
            .get(url)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}

impl Session<Disconnected> {
    pub fn connect(self, iv: [u8; 16]) -> Session<Connected> {
        Session {
            session_id: self.session_id,
            request_count: self.request_count,
            key: self.key,
            iv,
            state: Connected,
        }
    }
}

pub struct FunctionContext<'a> {
    instance_url: &'a Url,
    http: &'a reqwest::Client,
    function: Function,
}

impl<'a> FunctionContext<'a> {
    pub fn new(
        instance_url: &'a Url,
        http: &'a reqwest::Client,
        function: Function,
    ) -> FunctionContext<'a> {
        FunctionContext {
            instance_url,
            http,
            function,
        }
    }
}
