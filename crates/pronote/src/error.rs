use aes::cipher::block_padding;

#[derive(thiserror::Error, Debug)]
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
    #[error("error parsing datetime")]
    ParseDateTime(#[from] time::error::Parse),
    #[error("error formatting datetime")]
    FormatDateTime(#[from] time::error::Format),
    #[error("unknown lesson information kind")]
    UnknownLessonInformationKind { lesson_kind: u32 },
}
