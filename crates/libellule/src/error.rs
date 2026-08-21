#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not reach PRONOTE instance")]
    Transport(#[from] TransportError),

    #[error("unexpected anwser from PRONOTE")]
    Protocol(#[from] ProtocolError),

    #[error("authentication failed")]
    Authentication(#[from] AuthenticationError),

    #[error("could not interpret the data sent by PRONOTE")]
    Conversion(#[from] ConversionError),
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("http request failed")]
    Http(#[from] reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("no session identifier in landing page")]
    MissingSessionId,

    #[error("response body is not valid JSON")]
    InvalidJson(#[from] serde_json::Error),

    #[error("the session has expired")]
    SessionExpired,

    #[error("pronote server error `{code}`")]
    Server { code: i32, title: String },

    #[error("missing data field from response")]
    MissingData,

    #[error("unexpected value kind `{kind}` expected `{expected}`")]
    UnexpectedValueKind { kind: i32, expected: i32 },
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("login challenge is not valid hex")]
    BadChallenge,

    #[error("invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("failed to parse")]
    Parse,

    #[error("failed to parse date")]
    ParseDate(#[from] time::error::Parse),
}
