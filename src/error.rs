use std::error::Error as StdError;
use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Conversion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Conversion => "Failed to convert values",
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
