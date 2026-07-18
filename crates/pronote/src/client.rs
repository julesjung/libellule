use std::marker::PhantomData;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;
use serde_json::json;
use sha2::Digest;
use url::Url;

use crate::api::Function;
use crate::authentication::AuthenticationData;
use crate::crypto::{aes_decrypt, aes_encrypt};
use crate::error::Error;
use crate::identification::IndentificationData;
use crate::parameters::Parameters;
use crate::session::{FunctionContext, Session};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Disconnected;
#[derive(Debug)]
pub struct Connected;
#[derive(Debug)]
pub struct Authenticated;

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    instance_url: Url,
    http: reqwest::Client,
    session: Session,
    state: PhantomData<S>,
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
            state: PhantomData::<Disconnected>,
        })
    }

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

        session.iv = *md5::compute(iv);

        Ok((
            Client {
                instance_url: self.instance_url,
                http: self.http,
                session,
                state: PhantomData::<Connected>,
            },
            response,
        ))
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

impl Client<Connected> {
    pub async fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<(Client<Authenticated>, String), Error> {
        let mut session = self.session;

        let context =
            FunctionContext::new(&self.instance_url, &self.http, Function::Identification);

        let data = json!({
            "genreConnexion": 0,
            "genreEspace": 3,
            "identifiant": username,
            "pourENT": false,
            "enConnexionAuto": false,
            "demandeConnexionAuto": false,
            "enConnexionAppliMobile": false,
            "demandeConnexionAppliMobile": false,
            "demandeConnexionAppliMobileJeton": false,
            "uuidAppliMobile": "",
            "loginTokenSAV": "",
            "informationsAppareil": null
        });

        let response: IndentificationData = session.call(context, data).await?;

        let mut unencrypted_key = response.random;
        unencrypted_key.push_str(password);

        let mtp = hex::encode_upper(sha2::Sha256::digest(unencrypted_key.as_bytes()));

        let mut key = username.to_string();
        key.push_str(mtp.as_str());

        let key = md5::compute(key.as_bytes());

        let challenge = hex::decode(response.challenge)?;

        let decrypted_challenge = aes_decrypt(challenge.as_slice(), &key, &session.iv)?;
        let decrypted_challenge = String::from_utf8(decrypted_challenge).unwrap();

        let solution: String = decrypted_challenge
            .chars()
            .enumerate()
            .filter_map(|(index, character)| {
                if index % 2 == 0 {
                    Some(character)
                } else {
                    None
                }
            })
            .collect();

        let encrypted_solution = aes_encrypt(solution.as_bytes(), &key, &session.iv);
        let encrypted_solution = hex::encode(encrypted_solution);

        let context =
            FunctionContext::new(&self.instance_url, &self.http, Function::Authentication);

        let data = json!({
            "connexion": 0,
            "challenge": encrypted_solution,
            "espace": 3
        });

        let response: AuthenticationData = session.call(context, data).await?;

        Ok((
            Client {
                instance_url: self.instance_url,
                http: self.http,
                session,
                state: PhantomData::<Authenticated>,
            },
            response.fullname,
        ))
    }
}
