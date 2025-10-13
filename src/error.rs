use regex::Error as RegexError;
use std::fmt::Display;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
	IO(String),
	Parsing(String),
	Device(String),
	Package(String),
	Config(String),
	Installation(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::IO(e) => write!(f, "IO Error: {e}"),
			Error::Parsing(e) => write!(f, "Parsing Error: {e}"),
			Error::Device(e) => write!(f, "Device Error: {e}"),
			Error::Package(e) => write!(f, "Package Error: {e}"),
			Error::Config(e) => write!(f, "Config Error: {e}"),
			Error::Installation(e) => write!(f, "Installation Error: {e}"),
		}
	}
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
