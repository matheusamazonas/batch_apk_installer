use regex::{Error as RegexError, Regex};
use std::fmt::Display;
use std::{io, string};
use string::FromUtf8Error;

#[derive(Debug, PartialEq, Eq, Clone)]
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
	pub fn from_installation_error(error: &[u8]) -> Self {
		let error = String::from_utf8_lossy(error);
		if error.contains("INSTALL_FAILED_UPDATE_INCOMPATIBLE") {
			Self::PackageSignatureMismatch
		} else if error.contains("INSTALL_FAILED_VERSION_DOWNGRADE") {
			Self::PackageDowngrade
		} else if error.contains("INSTALL_FAILED_OLDER_SDK") {
			let regex = Regex::new(r"newer sdk version #(\d+).*current version is #(\d+)").unwrap();
			match regex.captures(&error) {
				None => Self::Installation(String::from(error)),
				Some(captures) if captures.len() < 2 => Self::Installation(String::from(error)),
				Some(captures) => {
					let app_sdk = captures[1].to_string();
					let device_sdk = captures[2].to_string();
					Self::OlderSDK(app_sdk, device_sdk)
				}
			}
		} else {
			Self::Installation(String::from(error))
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IO(e) => write!(f, "IO Error: {e}."),
			Self::MissingADB => write!(f, "ADB is missing."),
			Self::MissingAAPT => write!(f, "AAPT is missing."),
			Self::MissingPackagesFolderArgument => write!(f, "Missing argument: packages folder."),
			Self::NoHomeDirectory => write!(f, "No home directory found."),
			Self::NoPackageDirectory(e) => write!(f, "Missing package directory: {e}."),
			Self::Parsing(e) => write!(f, "Parsing Error: {e}."),
			Self::NoDeviceName => write!(f, "No device name provided."),
			Self::DevicesFetching => write!(f, "Failed to fetch devices."),
			Self::MalformedPackageFilePath => write!(f, "Package file path is not valid."),
			Self::PackageNameNotFound => write!(f, "Failed to fetch package name."),
			Self::ConfigNotFound => write!(f, "Config file not found."),
			Self::InvalidConfigPath => write!(f, "Invalid config path."),
			Self::Installation(e) => write!(f, "Installation error: {e}."),
			Self::PackageSignatureMismatch => write!(f, "APK signature mismatch."),
			Self::PackageDowngrade => write!(f, "Package downgrade."),
			Self::Uninstall(e) => write!(f, "Uninstall failed: {e}."),
			Self::WrongNumberOfArguments { actual, min, max } => write!(
				f,
				"Wrong number of arguments: {actual}. Expected between {min} and {max}. \
				Use -h to display the help text."
			),
			Self::UnknownArgument(e) => write!(f, "Unknown argument: {e}."),
			Self::NoDevices => write!(f, "No devices were found."),
			Self::NoPackages => write!(f, "No packages were found."),
			Self::OlderSDK(app_sdk, device_sdk) => write!(
				f,
				"APK's minimum API level ({app_sdk}) is higher than the device's (API level {device_sdk})."
			),
		}
	}
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::IO(e.to_string())
	}
}

impl From<FromUtf8Error> for Error {
	fn from(e: FromUtf8Error) -> Self {
		Self::Parsing(e.to_string())
	}
}

impl From<RegexError> for Error {
	fn from(e: RegexError) -> Self {
		Self::Parsing(e.to_string())
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Self::Parsing(e.to_string())
	}
}
