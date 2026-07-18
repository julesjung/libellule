use std::fmt::Debug;
use std::marker::PhantomData;

use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::{Function, Request, Response};
use crate::crypto::encode_request_count;
use crate::error::Error;

#[derive(Debug)]
pub struct Disconnected;
#[derive(Debug)]
pub struct Connected;

#[derive(Debug)]
pub struct Session<S = Disconnected> {
    session_id: u32,
    request_count: u32,
    key: [u8; 16],
    iv: [u8; 16],
    state: PhantomData<S>,
}

impl Session {
    pub fn new(session_id: u32) -> Session {
        Session {
            session_id,
            request_count: 0,
            key: *md5::compute([]),
            iv: [0u8; 16],
            state: PhantomData::<Disconnected>,
        }
    }

    pub fn encode_request_count(&self) -> String {
        encode_request_count(self.request_count, &self.key, &self.iv)
    }

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
            data,
        );

        let response: Response<D> = context
            .http
            .post(url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.data())
    }
}

impl Session<Disconnected> {
    pub fn connect(self, iv: [u8; 16]) -> Session<Connected> {
        Session {
            session_id: self.session_id,
            request_count: self.request_count,
            key: self.key,
            iv,
            state: PhantomData::<Connected>,
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
