use aes::cipher::block_padding;

#[derive(thiserror::Error, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi", uniffi(flat_error))]
pub enum Error {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("session id not found in response")]
    SessionIdNotFound,
    #[error("hex decoding error")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("incorrect password")]
    IncorrectPassword(#[from] block_padding::Error),
    #[error("unknown tab")]
    UnknownTab,
    #[error("invalid instace url")]
    InvalidUrl(#[from] url::ParseError),
}
