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
}
