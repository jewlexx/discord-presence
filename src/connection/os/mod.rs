//! OS-specific socket connection handling.

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub use unix::Socket;
    } else if #[cfg(windows)] {
        pub mod windows;
        pub use windows::Socket;
    }
}
