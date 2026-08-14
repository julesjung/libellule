use crate::models::MenuError;

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("{0}")]
    Menu(#[from] MenuError),
}
