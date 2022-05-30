use crossbeam_channel::SendError;
use serde_json::Error as JsonError;
use std::{
    fmt::{self, Display, Formatter},
    io::Error as IoError,
    result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
};
use thiserror::Error as AsError;

use crate::models::Message;

/// Error types from Discord
#[derive(Debug, AsError)]
pub enum Error {
    /// Io Error
    IoError(#[from] IoError),
    /// Communication between presence thread Error
    SendError(#[from] SendError<Message>),
    /// Json Error
    JsonError(#[from] JsonError),
    /// Timeout Error
    Timeout(#[from] ChannelTimeout),
    /// Conversion Error
    Conversion,
    /// Subscription Joining Error
    SubscriptionFailed,
    /// Connection Closing error
    ConnectionClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.description().as_str())
    }
}

impl Error {
    fn description(&self) -> String {
        match self {
            Error::Conversion => "Failed to convert values".into(),
            Error::SubscriptionFailed => "Failed to subscribe to event".into(),
            Error::ConnectionClosed => "Connection closed".into(),
            Error::IoError(ref err) => err.to_string(),
            Error::JsonError(ref err) => err.to_string(),
            Error::Timeout(ref err) => err.to_string(),
            Error::SendError(ref err) => err.to_string(),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::JsonError(err)
    }
}

impl From<ChannelTimeout> for Error {
    fn from(err: ChannelTimeout) -> Self {
        Error::Timeout(err)
    }
}

/// Result type for Discord RPC error types
pub type Result<T> = StdResult<T, Error>;
