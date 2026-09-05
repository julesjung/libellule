use md5::{Digest, Md5};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::crypto::aes_encrypt;
use crate::error::{Error, TransportError};
use crate::model::Tab;
use crate::protocol::{Function, Request, Response};

#[derive(Debug, Clone)]
pub struct Session {
    session_id: u32,
    request_count: u32,
    pub(crate) key: [u8; 16],
    pub(crate) iv: [u8; 16],
}

impl Session {
    pub fn new(session_id: u32) -> Session {
        Session {
            session_id,
            request_count: 0,
            key: Md5::digest([]).into(),
            iv: [0u8; 16],
        }
    }

    pub(crate) fn set_key<T>(&mut self, key: T)
    where
        T: AsRef<[u8]>,
    {
        self.key = Md5::digest(key).into()
    }

    pub(crate) fn set_iv<T>(&mut self, iv: T)
    where
        T: AsRef<[u8]>,
    {
        self.iv = Md5::digest(iv).into()
    }
}

impl Session {
    pub async fn call<'a, S, D>(
        &mut self,
        context: FunctionContext<'a>,
        data: S,
    ) -> Result<D, Error>
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
            context.tab,
            data,
        );

        let response = context
            .http
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(TransportError::from)?
            .text()
            .await
            .map_err(TransportError::from)?;

        self.request_count += 1;

        let response: Response<D> = serde_json::from_str(&response).unwrap();
        let data = response.into_data()?;

        Ok(data)
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
    tab: Option<Tab>,
}

impl<'a> FunctionContext<'a> {
    pub fn new(
        instance_url: &'a Url,
        http: &'a reqwest::Client,
        function: Function,
        tab: Option<Tab>,
    ) -> FunctionContext<'a> {
        FunctionContext {
            instance_url,
            http,
            function,
            tab,
        }
    }
}
