#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

mod authentication;
mod crypto;
mod identification;
mod protocol;
mod session;

pub mod client;
pub mod error;
pub mod instance;
pub mod models;
