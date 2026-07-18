use std::sync::Mutex;

use pronote::client::{Authenticated, Client as PronoteClient};
use pronote::client::{Connected, Disconnected};
use pronote::parameters::Parameters;
use url::Url;

uniffi::setup_scaffolding!();

enum ClientState {
    Disconnected(PronoteClient<Disconnected>),
    Connected(PronoteClient<Connected>),
    Authenticated(PronoteClient<Authenticated>),
}

#[derive(uniffi::Object)]
pub struct Client {
    state: Mutex<Option<ClientState>>,
}

#[derive(thiserror::Error, uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum Error {
    #[error("pronote error")]
    PronoteError(#[from] pronote::error::Error),
    #[error("incorrect client state")]
    IncorrectState,
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(instance_url: String) -> Result<Client, Error> {
        let instance_url = Url::parse(&instance_url)?;

        Ok(Client {
            state: Mutex::new(Some(ClientState::Disconnected(
                PronoteClient::from_url(instance_url).await?,
            ))),
        })
    }

    #[uniffi::method]
    pub async fn connect(&self) -> Result<ParametersRecord, Error> {
        let client = {
            let mut state = self.state.lock().unwrap();

            match state.take() {
                Some(ClientState::Disconnected(client)) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let (client, parameters) = client.connect().await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Connected(client));

        Ok(parameters.into())
    }

    #[uniffi::method]
    pub async fn authenticate(&self, username: String, password: String) -> Result<String, Error> {
        let client = {
            let mut state = self.state.lock().unwrap();

            match state.take() {
                Some(ClientState::Connected(client)) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let (client, fullname) = client.authenticate(&username, &password).await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Authenticated(client));
        
        Ok(fullname)
    }
}

#[derive(uniffi::Record)]
pub struct ParametersRecord {
    pub name: String,
    pub version: String,
}

impl From<Parameters> for ParametersRecord {
    fn from(value: Parameters) -> ParametersRecord {
        ParametersRecord {
            name: value.general.name,
            version: value.general.version,
        }
    }
}
