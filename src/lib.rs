use aes::Aes128;
use aes::cipher::{BlockModeEncrypt, KeyIvInit, block_padding::Pkcs7};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use cbc::Encryptor;
use rand::Rng;
use serde_json::json;
use thiserror::Error;
use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    instance_url: Url,
    http: reqwest::Client,
    state: S,
}

pub struct Disconnected;

#[derive(Debug)]
pub struct Connected {
    session_id: u32,
    request_count: u32,
    key: [u8; 16],
    iv: [u8; 16],
}

impl Client {
    pub fn from_url(mut instance_url: Url) -> Result<Client, reqwest::Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        instance_url.set_path("pronote/");

        Ok(Client {
            instance_url,
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

type Aes128CbcEnc = Encryptor<Aes128>;

fn encode_request_count(request_count: u32, key: &[u8; 16], iv: &[u8; 16]) -> String {
    let plaintext = request_count.to_string();

    let request_count =
        Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec::<Pkcs7>(plaintext.as_bytes());

    hex::encode(request_count)
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
            .get(self.instance_url.join("eleve.html").unwrap())
            .send()
            .await?
            .text()
            .await?;

        let session_id = extract_session_id(&response).ok_or(ConnectionError::NoSessionId)?;

        let mut request_count = 1;
        let key: [u8; 16] = *md5::compute(&[]);
        let mut iv = [0u8; 16];

        let encoded_request_count = encode_request_count(request_count, &key, &iv);

        let endpoint = format!("appelfonction/3/{}/{}", session_id, encoded_request_count);

        rand::rng().fill_bytes(&mut iv);

        let body = json!({
            "id": "FonctionParametres",
            "no": encoded_request_count,
            "session": session_id,
            "dataSec": {
                "data": {
                    "Uuid": STANDARD.encode(&iv),
                    "identifiantNav": null
                }
            }
        });

        let _response = self
            .http
            .post(self.instance_url.join(&endpoint).unwrap())
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        request_count += 2;

        Ok(Client {
            instance_url: self.instance_url,
            http: self.http,
            state: Connected {
                session_id,
                request_count,
                key,
                iv,
            },
        })
    }
}
