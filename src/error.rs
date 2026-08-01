use regex::{Error as RegexError, Regex};
use std::fmt::Display;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
	IO(String),
	NoHomeDirectory,
	Parsing(String),
	// Tools errors.
	MissingADB,
	MissingAAPT,
	// Package errors.
	NoPackages,
	NoPackageDirectory(String),
	MalformedPackageFilePath,
	PackageNameNotFound,
	// Device errors.
	NoDevices,
	NoDeviceName,
	DevicesFetching,
	// Config errors.
	ConfigNotFound,
	InvalidConfigPath,
	// Installation errors.
	Installation(String),
	PackageSignatureMismatch,
	PackageDowngrade,
	OlderSDK(String, String),
	// Uninstallation errors.
	Uninstall(String),
	// Argument errors.
	MissingPackagesFolderArgument,
	WrongNumberOfArguments {
		actual: usize,
		min: usize,
		max: usize,
	},
	UnknownArgument(String),
}

impl Error {
	pub fn from_installation_error(error: &[u8]) -> Error {
		let error = String::from_utf8_lossy(error);
		if error.contains("INSTALL_FAILED_UPDATE_INCOMPATIBLE") {
			Error::PackageSignatureMismatch
		} else if error.contains("INSTALL_FAILED_VERSION_DOWNGRADE") {
			Error::PackageDowngrade
		} else if error.contains("INSTALL_FAILED_OLDER_SDK") {
			let regex = Regex::new(r"newer sdk version #(\d+).*current version is #(\d+)").unwrap();
			match regex.captures(&error) {
				None => Error::Installation(String::from(error)),
				Some(captures) if captures.len() < 2 => Error::Installation(String::from(error)),
				Some(captures) => {
					let app_sdk = captures[1].to_string();
					let device_sdk = captures[2].to_string();
					Error::OlderSDK(app_sdk, device_sdk)
				}
			}
		} else {
			Error::Installation(String::from(error))
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::IO(e) => write!(f, "IO Error: {e}."),
			Error::MissingADB => write!(f, "ADB is missing."),
			Error::MissingAAPT => write!(f, "AAPT is missing."),
			Error::MissingPackagesFolderArgument => write!(f, "Missing argument: packages folder."),
			Error::NoHomeDirectory => write!(f, "No home directory found."),
			Error::NoPackageDirectory(e) => write!(f, "Missing package directory: {e}."),
			Error::Parsing(e) => write!(f, "Parsing Error: {e}."),
			Error::NoDeviceName => write!(f, "No device name provided."),
			Error::DevicesFetching => write!(f, "Failed to fetch devices."),
			Error::MalformedPackageFilePath => write!(f, "Package file path is not valid."),
			Error::PackageNameNotFound => write!(f, "Failed to fetch package name."),
			Error::ConfigNotFound => write!(f, "Config file not found."),
			Error::InvalidConfigPath => write!(f, "Invalid config path."),
			Error::Installation(e) => write!(f, "Installation error: {e}."),
			Error::PackageSignatureMismatch => write!(f, "APK signature mismatch."),
			Error::PackageDowngrade => write!(f, "Package downgrade."),
			Error::Uninstall(e) => write!(f, "Uninstall failed: {e}."),
			Error::WrongNumberOfArguments { actual, min, max } => write!(
				f,
				"Wrong number of arguments: {actual}. Expected between {min} and {max}. \
				Use -h to display the help text."
			),
			Error::UnknownArgument(e) => write!(f, "Unknown argument: {e}."),
			Error::NoDevices => write!(f, "No devices were found."),
			Error::NoPackages => write!(f, "No packages were found."),
			Error::OlderSDK(app_sdk, device_sdk) => write!(
				f,
				"APK's minimum API level ({app_sdk}) is higher than the device's (API level {device_sdk})."
			),
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
