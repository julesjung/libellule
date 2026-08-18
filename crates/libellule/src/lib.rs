//! # libellule
//!
//! The `libellule` crate provides a fast, type-safe PRONTE [`Client`].
//!
//! ## Example
//!
//! Add libellule as a dependency in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! libellule = "0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! And here is a usage example to get you started:
//!
//! ```rust
//! use libellule::{Client, Instance};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let instance_url = String::from("https://demo.index-education.net/pronote/");
//!     let instance = Instance::new(instance_url).await?;
//!     let username = "demonstration";
//!     let password = "pronotevs";
//!
//!     let client = Client::login(&instance, username, password).await?;
//!
//!     let periods = client.periods();
//!     let default_period_id = client.default_period();
//!     let default_period = periods
//!         .iter()
//!         .find(|period| period.id == default_period_id)
//!         .unwrap();
//!
//!     let grades = client.grades(default_period).await?;
//!
//!     println!("{:#?}", grades);
//!
//!     Ok(())
//! }
//! ```

mod client;
mod convert;
mod crypto;
mod instance;
mod protocol;
mod session;
mod time;

/// Error types used within the library.
pub mod error;

/// Improved representation of PRONOTE return types.
pub mod model;

pub use client::*;
pub use instance::*;
