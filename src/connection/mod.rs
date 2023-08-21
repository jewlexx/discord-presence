mod base;
mod manager;

pub use base::Connection;
pub use manager::Manager;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use unix::Socket;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::Socket;
    }
}
