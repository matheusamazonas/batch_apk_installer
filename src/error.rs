use std::{io, string};
use regex::Error as RegexError;
use string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArgumentError(String),
    AdbNotFound,
    IOError,
    ParseError,
    CommandError(String)
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::IOError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Error::ParseError
    }
}

impl  From<RegexError> for Error {
    fn from(_: RegexError) -> Self {
        Error::ParseError
    }
}