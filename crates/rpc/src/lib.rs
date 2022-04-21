#![warn(missing_docs)]

// Cannot remove this *macro_use*, would break derive inside of macros
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
pub mod client;
mod connection;
pub mod error;
mod event_handler;
pub mod models;
mod utils;

pub use self::client::Client;
pub use self::error::{Error, Result};
pub use self::models::Event;
