// Cannot remove this *macro_use*, would break derive inside of macros
#[macro_use] extern crate serde_derive;


#[macro_use]
mod macros;
mod utils;
mod connection;
mod event_handler;
pub mod error;
pub mod models;
pub mod client;

pub use self::client::Client;
pub use self::error::{Error, Result};
