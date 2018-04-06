use std::{
    error::Error as StdError,
    io::Error as IoError,
    result::Result as StdResult,
    fmt::{
        self,
        Display,
        Formatter
    }
};


#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Conversion,
    SubscriptionFailed,
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
            Error::Io(ref err) => err.description()
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = StdResult<T, Error>;
