#![feature(getpid)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byte;
extern crate uuid;

#[macro_use]
mod macros;

pub mod models;
pub mod client;

pub use client::Client;
