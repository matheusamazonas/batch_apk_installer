use regex::Error as RegexError;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum Error {
	ArgumentError(&'static str),
	AdbNotFound,
	IOError(String),
	ParseError(String),
	CommandError(&'static str),
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::IOError(e.to_string())
	}
}

impl From<FromUtf8Error> for Error {
	fn from(e: FromUtf8Error) -> Self {
		Error::ParseError(e.to_string())
	}
}

impl From<RegexError> for Error {
	fn from(e: RegexError) -> Self {
		Error::ParseError(e.to_string())
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Error::ParseError(e.to_string())
	}
}
