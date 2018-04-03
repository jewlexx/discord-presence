mod base;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::base::Connection as Connection;
#[cfg(unix)]
pub use self::unix::UnixConnection as SocketConnection;
#[cfg(windows)]
pub use self::windows::WindowsConnection as SocketConnection;
