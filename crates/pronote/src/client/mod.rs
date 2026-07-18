pub mod session;

use std::error::Error;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;
use serde_json::json;
use url::Url;

use crate::api::function::Function;
use crate::client::session::{Connected, Disconnected, FunctionContext, Session};
use crate::error::ConnectionError;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    instance_url: Url,
    http: reqwest::Client,
    session: Session<S>,
}

impl Client {
    pub async fn from_url(instance_url: Url) -> Result<Client, Box<dyn Error>> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        let response = http
            .get(instance_url.join("eleve.html").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let session_id = extract_session_id(&response).ok_or(ConnectionError::NoSessionId)?;

        let session = Session::new(session_id);

        Ok(Client {
            instance_url,
            http,
            session,
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

impl Client<Disconnected> {
    pub async fn connect(self) -> Result<Client<Connected>, session::Error> {
        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);

        let data = json!({"data": {
                "Uuid": STANDARD.encode(iv),
                "identifiantNav": null
            }
        });

        let context =
            FunctionContext::new(&self.instance_url, &self.http, Function::InstanceParameters);

        let mut session = self.session;

        session.call(context, data).await?;

        let session = session.connect(iv);

        Ok(Client {
            instance_url: self.instance_url,
            http: self.http,
            session,
        })
    }
}
