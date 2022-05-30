use crossbeam_channel::{RecvError, SendError};
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
pub enum DiscordError {
    /// Io Error
    IoError(#[from] IoError),
    /// Communication between presence thread Error
    SendError(#[from] SendError<Message>),
    /// Error Receiving message
    ReceiveError(#[from] RecvError),
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

impl Display for DiscordError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.description().as_str())
    }
}

impl DiscordError {
    fn description(&self) -> String {
        self.to_string()
    }
}

/// Result type for Discord RPC error types
pub type Result<T> = StdResult<T, DiscordError>;
