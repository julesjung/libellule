use std::time::Duration;

use base64::{Engine, engine::general_purpose::STANDARD};
use rand::Rng;
use reqwest::IntoUrl;
use serde_json::json;
use url::Url;

use crate::{
    error::{Error, ProtocolError, TransportError},
    protocol::{Function, InstanceParameters},
    session::{FunctionContext, Session},
};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

/// Represents a connection with a PRONOTE instance.
#[derive(Debug)]
pub struct Instance {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) session: Session,
    pub(crate) parameters: InstanceParameters,
}

impl Instance {
    /// Connects to a PRONOTE instance at a `url`.
    pub async fn new<T>(url: T) -> Result<Self, Error>
    where
        T: IntoUrl,
    {
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(TransportError::from)?;

        let base_url = url.into_url().map_err(TransportError::from)?;

        let response = http
            .get(base_url.join("eleve.html").unwrap())
            .send()
            .await
            .map_err(TransportError::from)?
            .text()
            .await
            .map_err(TransportError::from)?;

        let session_id = extract_session_id(&response)
            .ok_or(Error::Protocol(ProtocolError::MissingSessionId))?;

        let mut session = Session::new(session_id);

        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);

        let data = json!({
            "Uuid": STANDARD.encode(iv),
            "identifiantNav": null
        });

        let context = FunctionContext::new(&base_url, &http, Function::InstanceParameters, None);

        let parameters: InstanceParameters = session.call(context, data).await?;

        session.set_iv(iv);

        Ok(Instance {
            http,
            base_url,
            session,
            parameters,
        })
    }

    /// Returns the instance name.
    pub fn label(&self) -> &str {
        self.parameters.general.label.as_str()
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
