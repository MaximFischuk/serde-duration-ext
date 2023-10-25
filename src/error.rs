use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    Syntax(String),
    UnitNotSupported(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::Syntax(err) => err.fmt(f),
            Error::UnitNotSupported(err) => err.fmt(f),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Syntax(msg.to_string())
    }
}
