use std::{
    error::Error as StdError,
    io::Error as IoError,
    result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
    fmt::{
        self,
        Display,
        Formatter
    },
};
use serde_json::Error as JsonError;


#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    JsonError(JsonError),
    Timeout(ChannelTimeout),
    Conversion,
    SubscriptionFailed,
    ConnectionClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Conversion => "Failed to convert values",
            Error::SubscriptionFailed => "Failed to subscribe to event",
            Error::ConnectionClosed => "Connection closed",
            Error::IoError(ref err) => err.description(),
            Error::JsonError(ref err) => err.description(),
            Error::Timeout(ref err) => err.description(),
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

pub type Result<T> = StdResult<T, Error>;
