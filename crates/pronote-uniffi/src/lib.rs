use std::sync::Mutex;

use pronote::client::{Authenticated, Client as PronoteClient};
use pronote::client::{Connected, Disconnected};
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

#[derive(uniffi::Enum)]
pub enum ClientStatus {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Requesting,
}

enum ClientState {
    Disconnected(PronoteClient<Disconnected>),
    Connecting,
    Connected(PronoteClient<Connected>),
    Authenticating,
    Authenticated(PronoteClient<Authenticated>),
    Requesting,
}

#[derive(uniffi::Object)]
pub struct Client {
    state: Mutex<ClientState>,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(instance_url: String) -> Result<Client, Error> {
        let instance_url = Url::parse(&instance_url)?;

        Ok(Client {
            state: Mutex::new(ClientState::Disconnected(
                PronoteClient::from_url(instance_url).await?,
            )),
        })
    }

    pub fn status(&self) -> ClientStatus {
        let state = self.state.lock().unwrap();

        match &*state {
            ClientState::Disconnected(_) => ClientStatus::Disconnected,
            ClientState::Connecting => ClientStatus::Connecting,
            ClientState::Connected(_) => ClientStatus::Connected,
            ClientState::Authenticating => ClientStatus::Authenticating,
            ClientState::Authenticated(_) => ClientStatus::Authenticated,
            ClientState::Requesting => ClientStatus::Requesting,
        }
    }
}

impl Client {
    fn take_authenticated(&self) -> Result<PronoteClient<Authenticated>, Error> {
        let mut state = self.state.lock().unwrap();

        match std::mem::replace(&mut *state, ClientState::Requesting) {
            ClientState::Authenticated(client) => Ok(client),
            other => {
                *state = other;
                return Err(Error::IncorrectState);
            }
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn connect(&self) -> Result<(), Error> {
        let client = {
            let mut state = self.state.lock().unwrap();

            match std::mem::replace(&mut *state, ClientState::Connecting) {
                ClientState::Disconnected(client) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let client = client.connect().await?;

        let mut state = self.state.lock().unwrap();
        *state = ClientState::Connected(client);

        Ok(())
    }

    pub async fn authenticate(&self, username: String, password: String) -> Result<(), Error> {
        let client = {
            let mut state = self.state.lock().unwrap();

            match std::mem::replace(&mut *state, ClientState::Authenticating) {
                ClientState::Connected(client) => client,
                other => {
                    *state = other;
                    return Err(Error::IncorrectState);
                }
            }
        };

        let client = client.authenticate(&username, &password).await?;

        let mut state = self.state.lock().unwrap();
        *state = ClientState::Authenticated(client);

        Ok(())
    }

    pub async fn user_information(&self) -> Result<User, Error> {
        let mut client = self.take_authenticated()?;

        let user = client.user_information().await?;

        let mut state = self.state.lock().unwrap();
        *state = ClientState::Authenticated(client);

        Ok(user.into())
    }
}

#[derive(Debug, uniffi::Record)]
pub struct User {
    pub fullname: String,
    pub institution_name: String,
    pub class: String,
    pub profile_picture: String,
}

impl From<pronote::models::User> for User {
    fn from(value: pronote::models::User) -> User {
        User {
            fullname: value.fullname,
            institution_name: value.institution_name,
            class: value.class,
            profile_picture: value.profile_picture,
        }
    }
}
