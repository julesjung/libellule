use std::fmt::Display;

#[derive(uniffi::Error, Debug, thiserror::Error)]
#[uniffi(flat_error)]
pub enum LibelluleError {
    LibelluleError(#[from] libellule::error::Error),
}

impl Display for LibelluleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
