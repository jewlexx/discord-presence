mod base;

#[cfg(unix)]
mod unix;

pub use self::base::Connection;
#[cfg(unix)]
pub use self::unix::UnixConnection;
