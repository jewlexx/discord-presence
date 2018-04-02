mod base;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::base::Connection;
#[cfg(unix)]
pub use self::unix::UnixConnection;
#[cfg(windows)]
pub use self::windows::WindowsConnection;
