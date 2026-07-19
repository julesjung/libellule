use std::marker::PhantomData;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::Rng;
use serde_json::json;
use sha2::Digest;
use url::Url;

use crate::api::{self, Empty, Function, Response};
use crate::authentication::AuthenticationData;
use crate::crypto::{aes_decrypt, aes_encrypt};
use crate::error::Error;
use crate::identification::IndentificationData;
use crate::models;
use crate::session::{FunctionContext, Session};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Disconnected;
#[derive(Debug)]
pub struct Connected;
#[derive(Debug)]
pub struct Authenticated;
#[derive(Debug)]
pub struct Ready;

#[derive(Debug)]
pub struct Client<S = Disconnected> {
    instance_url: Url,
    http: reqwest::Client,
    session: Session,
    parameters: Option<api::UserParameters>,
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
            parameters: None,
            state: PhantomData::<Disconnected>,
        })
    }

    pub async fn connect(self) -> Result<Client<Connected>, Error> {
        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);

        let data = json!({
            "Uuid": STANDARD.encode(iv),
            "identifiantNav": null
        });

        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::InstanceParameters,
            None,
        );

        let mut session = self.session;

        let _: Response<Empty> = session.call(context, data).await?;

        session.iv = *md5::compute(iv);

        Ok(Client {
            instance_url: self.instance_url,
            http: self.http,
            session,
            parameters: None,
            state: PhantomData::<Connected>,
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

impl Client<Connected> {
    pub async fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Client<Authenticated>, Error> {
        let mut session = self.session;

        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::Identification,
            None,
        );

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

        let response: Response<IndentificationData> = session.call(context, data).await?;
        let data = response.secured_data.data;

        let mut unencrypted_key = data.random;
        unencrypted_key.push_str(password);

        let mtp = hex::encode_upper(sha2::Sha256::digest(unencrypted_key.as_bytes()));

        let mut temporary_key = username.to_string();
        temporary_key.push_str(mtp.as_str());

        let temporary_key = md5::compute(temporary_key.as_bytes());

        let challenge = hex::decode(data.challenge)?;

        let decrypted_challenge = aes_decrypt(challenge.as_slice(), &temporary_key, &session.iv)?;
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

        let encrypted_solution = aes_encrypt(solution.as_bytes(), &temporary_key, &session.iv);
        let encrypted_solution = hex::encode(encrypted_solution);

        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::Authentication,
            None,
        );

        let data = json!({
            "connexion": 0,
            "challenge": encrypted_solution,
            "espace": 3
        });

        let response: Response<AuthenticationData> = session.call(context, data).await?;
        let data = response.secured_data.data;

        let encrypted_key = hex::decode(&data.key)?;
        let new_key = aes_decrypt(encrypted_key.as_slice(), &temporary_key, &session.iv)?;
        let new_key: Vec<u8> = String::from_utf8(new_key)
            .unwrap()
            .split(',')
            .map(|byte| byte.parse::<u8>().unwrap())
            .collect();

        session.key = *md5::compute(new_key);

        Ok(Client {
            instance_url: self.instance_url,
            http: self.http,
            session,
            parameters: None,
            state: PhantomData::<Authenticated>,
        })
    }
}

impl Client<Authenticated> {
    pub async fn load_user(self) -> Result<Client<Ready>, Error> {
        let mut session = self.session;

        let context = FunctionContext::new(
            &self.instance_url,
            &self.http,
            Function::UserParameters,
            None,
        );

        let response: Response<api::UserParameters> = session.call(context, Empty::new()).await?;

        Ok(Client {
            instance_url: self.instance_url,
            http: self.http,
            session,
            parameters: Some(response.secured_data.data),
            state: PhantomData::<Ready>,
        })
    }
}

impl Client<Ready> {
    pub async fn get_grades(&mut self) -> Result<models::GradesData, Error> {
        let context =
            FunctionContext::new(&self.instance_url, &self.http, Function::Grades, Some(198));

        let period = self
            .parameters
            .as_ref()
            .unwrap()
            .resources
            .tab_periods_list
            .value
            .iter()
            .find_map(|tab_periods| match tab_periods.id {
                198 => Some(&tab_periods.default_period.value),
                _ => None,
            })
            .unwrap();

        let data = json!({
            "Periode": period
        });

        let response: Response<api::GradesData> = self.session.call(context, data).await?;

        Ok(response.secured_data.data.into())
    }
}
