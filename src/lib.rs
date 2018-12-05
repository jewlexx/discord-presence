#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate byteorder;
extern crate uuid;
extern crate bytes;
extern crate parking_lot;
extern crate crossbeam_channel;
#[cfg(windows)]
extern crate named_pipe;

#[macro_use]
mod macros;
mod error;
mod utils;
mod connection;
pub mod models;
pub mod client;

pub use client::Client;
pub use connection::{Connection, SocketConnection};
