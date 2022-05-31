mod base;
mod manager;

pub use base::Connection;
pub use manager::Manager;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use unix::UnixConnection as SocketConnection;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::WindowsConnection as SocketConnection;
    }
}
