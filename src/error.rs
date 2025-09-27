use regex::Error as RegexError;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum Error {
	IO(String),
	Parsing(String),
	Device(String),
	Package(String),
	Config(String),
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Error::IO(e.to_string())
	}
}

impl From<FromUtf8Error> for Error {
	fn from(e: FromUtf8Error) -> Self {
		Error::Parsing(e.to_string())
	}
}

impl From<RegexError> for Error {
	fn from(e: RegexError) -> Self {
		Error::Parsing(e.to_string())
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Error::Parsing(e.to_string())
	}
}
