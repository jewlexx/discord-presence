mod base;
mod manager;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use self::unix::UnixConnection as SocketConnection;
#[cfg(windows)]
pub use self::windows::WindowsConnection as SocketConnection;
pub use self::{base::Connection, manager::Manager};
