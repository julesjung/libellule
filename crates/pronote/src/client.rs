use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;
use serde_json::json;
use url::Url;

use crate::api::Function;
use crate::error::Error;
use crate::parameters::Parameters;
use crate::session::{Connected, Disconnected, FunctionContext, Session};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    pub(crate) instance_url: Url,
    pub(crate) http: reqwest::Client,
    pub(crate) session: Session<S>,
}

impl Client {
    pub async fn from_url(instance_url: Url) -> Result<Client, Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        let response = http
            .get(instance_url.join("eleve.html").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let session_id = extract_session_id(&response).ok_or(Error::NoSessionId)?;

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
    pub async fn connect(self) -> Result<(Client<Connected>, Parameters), Error> {
        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);

        let data = json!({
            "Uuid": STANDARD.encode(iv),
            "identifiantNav": null
        });

        let context =
            FunctionContext::new(&self.instance_url, &self.http, Function::InstanceParameters);

        let mut session = self.session;

        let response: Parameters = session.call(context, data).await?;

        let session = session.connect(iv);

        Ok((
            Client {
                instance_url: self.instance_url,
                http: self.http,
                session,
            },
            response,
        ))
    }
}
