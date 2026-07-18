use std::sync::Mutex;

use pronote::client::Client as PronoteClient;
use pronote::parameters::Parameters;
use pronote::{Connected, Disconnected};
use url::Url;

uniffi::setup_scaffolding!();

enum ClientState {
    Disconnected(PronoteClient<Disconnected>),
    Connected(PronoteClient<Connected>),
}

#[derive(uniffi::Object)]
pub struct Client {
    state: Mutex<Option<ClientState>>,
}

#[derive(uniffi::Error, Debug)]
pub enum Error {
    IncorrectUrl,
    WrongState,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IncorrectUrl => write!(f, "incorrect URL"),
            Error::WrongState => write!(f, "a method was called in the wrong state"),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(instance_url: String) -> Result<Client, Error> {
        let instance_url = Url::parse(&instance_url).map_err(|_| Error::IncorrectUrl)?;

        Ok(Client {
            state: Mutex::new(Some(ClientState::Disconnected(
                PronoteClient::from_url(instance_url)
                    .await
                    .map_err(|_| Error::IncorrectUrl)?,
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
                    return Err(Error::WrongState);
                }
            }
        };

        let (client, parameters) = client.connect().await.map_err(|_| Error::IncorrectUrl)?;

        let mut state = self.state.lock().unwrap();
        *state = Some(ClientState::Connected(client));

        Ok(parameters.into())
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
