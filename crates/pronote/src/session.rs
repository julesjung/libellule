use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::function::{Function, Request, Response};
use crate::crypto::aes_encrypt;
use crate::error::Error;

#[derive(Debug)]
pub struct Session {
    pub session_id: u32,
    pub request_count: u32,
    pub key: [u8; 16],
    pub iv: [u8; 16],
}

impl Session {
    pub fn new(session_id: u32) -> Session {
        Session {
            session_id,
            request_count: 0,
            key: *md5::compute([]),
            iv: [0u8; 16],
        }
    }
}

impl Session {
    pub async fn call<'a, S, D>(
        &mut self,
        context: FunctionContext<'a>,
        data: S,
    ) -> Result<Response<D>, Error>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        self.request_count += 1;
        let encoded_request_count = self.encode_request_count();

        let endpoint = format!(
            "appelfonction/3/{}/{}",
            self.session_id, encoded_request_count
        );

        let url = context.instance_url.join(&endpoint).unwrap();

        let body = Request::new(
            context.function,
            encoded_request_count,
            self.session_id,
            data,
        );

        let response = context
            .http
            .post(url)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        self.request_count += 1;

        let response: Response<D> = serde_json::from_str(&response).unwrap();

        Ok(response)
    }

    pub fn encode_request_count(&self) -> String {
        let plaintext = self.request_count.to_string();
        let cipher = aes_encrypt(plaintext.as_bytes(), &self.key, &self.iv);
        hex::encode(cipher)
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
