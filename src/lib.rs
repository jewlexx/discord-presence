#![warn(
    missing_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic
)]
// #![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! A Rust library that allows the developer to interact with the Discord Presence API with ease

use std::sync::atomic::AtomicBool;

pub use client::Client;
pub use error::{DiscordError, Result};
pub use models::Event;

pub(crate) static READY: AtomicBool = AtomicBool::new(false);

// Cannot remove this *macro_use*, would break derive inside of macros
#[macro_use]
extern crate serde;

#[macro_use]
extern crate log;

#[macro_use]
mod macros;
/// A client for the Discord Presence API
pub mod client;
/// Connection handling for the Discord Presence API
pub mod connection;
/// Errors that can occur when interacting with the Discord Presence API
pub mod error;
/// Event handlers
pub mod event_handler;
/// Models for discord activity
pub mod models;

pub(crate) fn nonce() -> String {
    uuid::Uuid::new_v4().to_string()
}
