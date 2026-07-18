use aes::cipher::block_padding;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("session id not found in response")]
    NoSessionId,
    #[error("hex decoding error")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("incorrect password")]
    IncorrectPassword(#[from] block_padding::Error),
}
