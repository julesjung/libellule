use thiserror::Error;
use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    endpoint: Url,
    http: reqwest::Client,
    state: S,
}

pub struct Disconnected;

#[derive(Debug)]
pub struct Connected {
    session_id: u32,
}

impl Client {
    pub fn new(endpoint: Url) -> Result<Client, reqwest::Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        Ok(Client {
            endpoint,
            http,
            state: Disconnected,
        })
    }
}

fn extract_session_id(input: &str) -> Option<u32> {
    let start = input.find("Start")?;
    let input = &input[start..];

    let open = input.find('{')? + 1;
    let close = input.find('}')?;
    let inner = &input[open..close];

    let mut session_id = None;

    for pair in inner.split(',') {
        let (key, value) = pair.split_once(":")?;

        if key.contains("h") {
            session_id = Some(value.parse().ok()?)
        }
    }

    session_id
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("session id not found in response")]
    NoSessionId,
}

impl Client<Disconnected> {
    pub async fn connect(self) -> Result<Client<Connected>, ConnectionError> {
        let response = self
            .http
            .get(self.endpoint.clone())
            .send()
            .await?
            .text()
            .await?;

        let session_id = extract_session_id(&response).ok_or(ConnectionError::NoSessionId)?;

        Ok(Client {
            endpoint: self.endpoint,
            http: self.http,
            state: Connected { session_id },
        })
    }
}
