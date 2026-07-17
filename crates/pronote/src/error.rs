use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("session id not found in response")]
    NoSessionId,
}
