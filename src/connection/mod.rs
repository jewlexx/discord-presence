mod base;
mod manager;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub use self::base::Connection as Connection;
pub use self::manager::Manager;
#[cfg(unix)]
pub use self::unix::UnixConnection as SocketConnection;
#[cfg(windows)]
pub use self::windows::WindowsConnection as SocketConnection;
