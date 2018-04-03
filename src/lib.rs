#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byteorder;
extern crate uuid;
extern crate libc;
#[cfg(windows)]
extern crate named_pipe;

#[macro_use]
mod macros;
mod error;
mod utils;
mod connection;
mod models;
mod rich_presence;

pub mod client;

pub use client::Client;
#[cfg(feature = "rich_presence")]
pub use rich_presence::*;
#[cfg(unix)]
pub use connection::UnixConnection;
#[cfg(windows)]
pub use connection::WindowsConnection;
