#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("session id not found in response")]
    NoSessionId,
}
