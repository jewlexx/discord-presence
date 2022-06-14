use crossbeam_channel::{RecvError, RecvTimeoutError, SendError, TryRecvError};
use serde_json::Error as JsonError;
use std::{
    io::Error as IoError, result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
};
use thiserror::Error as AsError;

use crate::models::Message;

/// Error types from Discord
#[derive(Debug, AsError)]
pub enum DiscordError {
    /// Io Error
    #[error("Io Error")]
    IoError(#[from] IoError),
    /// Communication Error between presence thread
    #[error("Communication Error between presence thread")]
    SendError(#[from] SendError<Message>),
    /// Error Receiving message
    #[error("Error Receiving message")]
    ReceiveError(#[from] RecvError),
    /// Json Error
    #[error("Error parsing Json")]
    JsonError(#[from] JsonError),
    /// Timeout Error
    #[error("Error on Channel Timeout")]
    Timeout(#[from] ChannelTimeout),
    /// No Message Sent
    #[error("No message sent")]
    RecvError(#[from] TryRecvError),
    /// Receiving timed out
    #[error("Recieving timed out")]
    RecvTimeoutError(#[from] RecvTimeoutError),
    /// Option unwrapped to None
    #[error("{0}")]
    NoneError(String),
    /// Conversion Error
    #[error("Error converting values")]
    Conversion,
    /// Subscription Joining Error
    #[error("Error subscribing to an event")]
    SubscriptionFailed,
    /// Connection Closing error
    #[error("Connection was closed prematurely")]
    ConnectionClosed,
}

impl DiscordError {
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
