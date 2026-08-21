/// Errors returned by the client while talking to a PRONOTE instance.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The request never reached PRONOTE.
    #[error("could not reach PRONOTE instance")]
    Transport(#[from] TransportError),

    /// PRONOTE answered with unexpected data.
    #[error("unexpected anwser from PRONOTE")]
    Protocol(#[from] ProtocolError),

    /// The credentials or the login challenge were rejected.
    #[error("authentication failed")]
    Authentication(#[from] AuthenticationError),

    /// The answer could not be mapped to the model.
    #[error("could not interpret the data sent by PRONOTE")]
    Conversion(#[from] ConversionError),
}

/// The request never reached PRONOTE. A retry might help.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    /// The request failed.
    #[error("http request failed")]
    Http(#[from] reqwest::Error),
}

/// PRONOTE answere with something that we did not ask for. The user might need to log back in.
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    /// No session id was found in the landing page. Make sure that the given url is a valid PRONOTE instance.
    #[error("no session identifier in landing page")]
    MissingSessionId,

    /// The response could not be parsed as JSON.
    #[error("response body is not valid JSON")]
    InvalidJson(#[from] serde_json::Error),

    /// The session has expired. The user needs to log back in.
    #[error("the session has expired")]
    SessionExpired,

    /// An error was sent by the PRONOTE server.
    #[error("pronote server error `{code}`")]
    Server {
        /// The error code.
        code: i32,

        /// The title given by PRONOTE.
        title: String,
    },

    /// No data was found in the response.
    #[error("missing data field from response")]
    MissingData,

    /// The wrong value kind was received.
    #[error("unexpected value kind `{kind}` expected `{expected}`")]
    UnexpectedValueKind {
        /// The value kind that was sent
        kind: i32,

        /// The value kind that was expected
        expected: i32,
    },
}

/// The credentials or the login challenge were rejected. The user might need to type in something a different.
#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    /// The challenge given by PRONOTE's server is invalid.
    #[error("login challenge is not valid hex")]
    BadChallenge,

    /// The challenge could not be decrypted with the user's password. Most of the time, it means that the username or the password is wrong.
    #[error("invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, thiserror::Error)]
/// The answer could not be mapped to the model. If this error happens, report a bug.
pub enum ConversionError {
    /// Some data could not be parsed.
    #[error("failed to parse")]
    Parse,

    /// The date was in an invalid format.
    #[error("failed to parse date")]
    ParseDate(#[from] time::error::Parse),
}
