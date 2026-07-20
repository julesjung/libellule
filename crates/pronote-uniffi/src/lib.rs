use std::sync::Mutex;

use pronote::client::{Authenticated, Client as PronoteClient, Connected, Disconnected, Ready};
use pronote::models::GradesData;
use url::Url;

uniffi::setup_scaffolding!();

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

enum ClientState {
    Disconnected(PronoteClient<Disconnected>),
    Connected(PronoteClient<Connected>),
    Authenticated(PronoteClient<Authenticated>),
    Ready(PronoteClient<Ready>),
}

#[derive(uniffi::Object)]
pub struct Client {
    state: Mutex<Option<ClientState>>,
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
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn connect(&self) -> Result<(), Error> {
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

        let client = client.connect().await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Connected(client));

        Ok(())
    }

    pub async fn authenticate(&self, username: String, password: String) -> Result<(), Error> {
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

        let client = client.authenticate(&username, &password).await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Authenticated(client));

        Ok(())
    }

    pub async fn load_user(&self) -> Result<(), Error> {
        let client = {
            let mut state = self.state.lock().unwrap();

            match state.take() {
                Some(ClientState::Authenticated(client)) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let client = client.load_user().await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Ready(client));

        Ok(())
    }

    pub async fn get_grades(&self) -> Result<GradesData, Error> {
        let mut client = {
            let mut state = self.state.lock().unwrap();

            match state.take() {
                Some(ClientState::Ready(client)) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let grades_data = client.get_grades().await?;

        Ok(grades_data.into())
    }
}
