#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

mod api;
mod authentication;
mod crypto;
mod identification;
mod session;

pub mod client;
pub mod error;
pub mod instance;
pub mod models;
