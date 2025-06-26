mod base;
mod manager;

pub mod location;

use base::Connection;
pub(crate) use manager::Manager;

mod os;

pub use os::Socket;
