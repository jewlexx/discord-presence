#![feature(getpid)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byte;
extern crate uuid;

#[macro_use]
mod macros;

mod connection;
mod models;
pub mod client;

pub use client::Client;
pub use models::prelude;

#[cfg(unix)]
pub use connection::UnixConnection;
