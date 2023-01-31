mod base;
mod manager;

pub use base::Connection;
pub use manager::Manager;

cfg_if::cfg_if! {
    if #[cfg(all(unix, feature = "ipc"))]  {
        mod unix;
        pub use unix::UnixConnection as SocketConnection;
    } else if #[cfg(all(windows, feature = "ipc"))] {
        mod windows;
        pub use windows::WindowsConnection as SocketConnection;
    }
     else if #[cfg(feature = "ws")] {
        mod socket;
        pub use socket::SocketConnection;
    }
}
