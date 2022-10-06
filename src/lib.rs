#![warn(missing_docs, rust_2018_compatibility, rust_2018_idioms, clippy::all)]
#![forbid(unsafe_code)]

//! A Rust library that allows the developer to interact with the Discord Presence API with ease

pub(crate) static STARTED: AtomicBool = AtomicBool::new(false);
pub(crate) static READY: AtomicBool = AtomicBool::new(false);

lazy_static! {
    pub(crate) static ref RATE_LIMIT_CHECK: Mutex<(Instant, u8)> = Mutex::new((Instant::now(), 0));
}

// Cannot remove this *macro_use*, would break derive inside of macros
#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[macro_use]
mod macros;
/// A client for the Discord Presence API
pub mod client;
mod connection;
/// Errors that can occur when interacting with the Discord Presence API
pub mod error;
mod event_handler;
/// Models for discord activity
pub mod models;
mod utils;

use std::{sync::atomic::AtomicBool, time::Instant};

pub use client::Client;
pub use error::{DiscordError, Result};
use lazy_static::lazy_static;
pub use models::Event;
use parking_lot::Mutex;
