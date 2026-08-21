use libellule::error::{
    AuthenticationError, ConversionError, Error, ProtocolError, TransportError,
};

type LibelluleError = Error;

#[uniffi::remote(Error)]
pub enum LibelluleError {
    Transport(TransportError),
    Protocol(ProtocolError),
    Authentication(AuthenticationError),
    Conversion(ConversionError),
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum TransportError {
    Http(_),
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ProtocolError {
    MissingSessionId,
    InvalidJson(_),
    SessionExpired,
    Server { code: i32, title: String },
    MissingData,
    UnexpectedValueKind { kind: i32, expected: i32 },
}

#[uniffi::remote(Error)]
pub enum AuthenticationError {
    BadChallenge,
    InvalidCredentials,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ConversionError {
    Parse,
    ParseDate(_),
}
