use std::sync::Mutex;

use pronote::client::Client as PronoteClient;
use pronote::models::GradesData;
use url::Url;

uniffi::setup_scaffolding!();

#[derive(thiserror::Error, uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum Error {
    #[error("pronote error")]
    PronoteError(#[from] pronote::error::Error),
    #[error("client is already being used")]
    AlreadyInUse,
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(uniffi::Object)]
pub struct Client {
    inner: Mutex<Option<PronoteClient>>,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(
        instance_url: String,
        password: &str,
        username: &str,
    ) -> Result<Client, Error> {
        let instance_url = Url::parse(&instance_url)?;
        let client = PronoteClient::login(instance_url, password, username).await?;

        Ok(Client {
            inner: Mutex::new(Some(client)),
        })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn get_grades(&self) -> Result<GradesData, Error> {
        let mut client = {
            let client = self.inner.lock().unwrap();

            client.take().ok_or(Error::AlreadyInUse)?
        };

        let grades_data = client.get_grades().await?;

        *self.inner.lock().unwrap() = Some(client);

        Ok(grades_data.into())
    }
}
