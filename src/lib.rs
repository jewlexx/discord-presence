#![feature(getpid)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byte;
extern crate uuid;

mod models;
mod client;

pub use client::Client;
