use crossbeam_channel::{RecvError, RecvTimeoutError, SendError};
use serde_json::Error as JsonError;
use std::{
    io::Error as IoError, result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
};

use crate::models::Message;

/// Error types from Discord
#[derive(Debug, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum DiscordError {
    #[error("Io Error")]
    /// Io Error
    IoError(#[from] IoError),
    #[error("Could not send message: {0}")]
    /// tx.send returned error
    SendMessage(#[from] SendError<Message>),
    #[error("Could not close event loop: {0}")]
    /// tx.send returned error
    CloseError(#[from] SendError<()>),
    #[error("Error Receiving message")]
    /// Error Receiving message
    ReceiveError(#[from] RecvError),
    #[error("Error parsing Json")]
    /// Json Error
    JsonError(#[from] JsonError),
    #[error("Error on Channel Timeout")]
    /// Timeout Error
    Timeout(#[from] ChannelTimeout),
    #[error("Receiving timed out")]
    /// Receiving timed out
    RecvTimeoutError(#[from] RecvTimeoutError),
    #[error("{0}")]
    /// Option unwrapped to None
    NoneError(String),
    #[error("Error converting values")]
    /// Conversion Error
    Conversion,
    #[error("Error subscribing to an event")]
    /// Subscription Joining Error
    SubscriptionFailed,
    #[error("Connection was closed prematurely")]
    /// Connection Closing error
    ConnectionClosed,
    #[error("Connection has not been started")]
    /// Connection has not been started
    NotStarted,
    #[error("Event loop ran into an unknown error")]
    /// The send & receive loop ran into an error
    EventLoopError,
}

impl DiscordError {
    #[must_use]
    /// Tell whether an [`IoError`] would block the connection
    pub fn io_would_block(&self) -> bool {
        match self {
            Self::IoError(ref err) => err.kind() == std::io::ErrorKind::WouldBlock,
            _ => false,
        }
    }
}

/// Result type for Discord RPC error types
pub type Result<T> = StdResult<T, DiscordError>;
