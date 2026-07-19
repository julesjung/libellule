use std::sync::Mutex;

use pronote::client::{Authenticated, Client as PronoteClient};
use pronote::client::{Connected, Disconnected};
use pronote::models::User;
use url::Url;

uniffi::include_scaffolding!("pronote");

enum ClientState {
    Disconnected(PronoteClient<Disconnected>),
    Connected(PronoteClient<Connected>),
    Authenticated(PronoteClient<Authenticated>),
}

pub struct Client {
    state: Mutex<Option<ClientState>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("pronote error")]
    PronoteError(#[from] pronote::error::Error),
    #[error("incorrect client state")]
    IncorrectState,
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

impl Client {
    pub async fn new(instance_url: String) -> Result<Client, Error> {
        let instance_url = Url::parse(&instance_url)?;

        Ok(Client {
            state: Mutex::new(Some(ClientState::Disconnected(
                PronoteClient::from_url(instance_url).await?,
            ))),
        })
    }

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

    pub async fn user_information(&self) -> Result<User, Error> {
        let mut client = {
            let mut state = self.state.lock().unwrap();

            match state.take() {
                Some(ClientState::Authenticated(client)) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let user = client.user_information().await?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Authenticated(client));

        Ok(user)
    }
}
