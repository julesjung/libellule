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
pub enum ProtocolError {
    MissingSessionId,
}

#[uniffi::remote(Error)]
pub enum AuthenticationError {
    BadChallenge,
    InvalidCredentials,
}

#[uniffi::remote(Error)]
pub enum ConversionError {
    Parse,
}
