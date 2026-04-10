use regex::Error as RegexError;
use std::fmt::Display;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
	IO(String),
	MissingADB,
	MissingAAPT,
	MissingPackagesFolderArgument,
	NoHomeDirectory,
	NoPackageDirectory(String),
	Parsing(String),
	NoDeviceName,
	DevicesFetching,
	MalformedPackageFilePath,
	PackageNameNotFound,
	ConfigNotFound,
	InvalidConfigPath,
	Installation(String),
	PackageSignatureMismatch,
	PackageDowngrade,
	Uninstall(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::IO(e) => write!(f, "IO Error: {e}"),
			Error::MissingADB => write!(f, "ADB is missing."),
			Error::MissingAAPT => write!(f, "AAPT is missing."),
			Error::MissingPackagesFolderArgument => write!(f, "Missing argument: packages folder."),
			Error::NoHomeDirectory => write!(f, "No home directory found"),
			Error::NoPackageDirectory(e) => write!(f, "Missing package directory: {e}"),
			Error::Parsing(e) => write!(f, "Parsing Error: {e}"),
			Error::NoDeviceName => write!(f, "No device name provided"),
			Error::DevicesFetching => write!(f, "Failed to fetch devices"),
			Error::MalformedPackageFilePath => write!(f, "Package file path is not valid"),
			Error::PackageNameNotFound => write!(f, "Failed to fetch package name"),
			Error::ConfigNotFound => write!(f, "Config file not found"),
			Error::InvalidConfigPath => write!(f, "Invalid config path"),
			Error::Installation(e) => write!(f, "Installation error: {e}"),
			Error::PackageSignatureMismatch => write!(f, "APK signature mismatch"),
			Error::PackageDowngrade => write!(f, "Package downgrade"),
			Error::Uninstall(e) => write!(f, "Uninstall failed: {e}"),
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
