use base64::{Engine, engine::general_purpose::STANDARD};
use rand::Rng;
use serde_json::json;
use url::Url;

use crate::{
    error::Error,
    protocol::{Empty, Function, Response},
    session::{FunctionContext, Session},
};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct Instance {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) session: Session,
}

#[cfg_attr(feature = "uniffi", uniffi::export(async_runtime = "tokio"))]
impl Instance {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub async fn new(url: String) -> Result<Self, Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        // TODO: parse url in a more meaningful way
        let base_url = Url::parse(&url)?;

        let response = http
            .get(base_url.join("eleve.html").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let session_id = extract_session_id(&response).ok_or(Error::SessionIdNotFound)?;

        let mut session = Session::new(session_id);

        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);

        let data = json!({
            "Uuid": STANDARD.encode(iv),
            "identifiantNav": null
        });

        let context = FunctionContext::new(&base_url, &http, Function::InstanceParameters, None);

        // TODO: parse instance information
        let _: Response<Empty> = session.call(context, data).await?;

        session.iv = *md5::compute(iv);

        Ok(Instance {
            http,
            base_url,
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
