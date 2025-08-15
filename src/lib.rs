use crate::error::Error::{CommandError, IOError};
use error::Error;
use regex::{self, Regex};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Write;

pub mod android;
pub mod error;

type Platform = String;
type AppId = String;
type Version = String;

const CONFIG_PATH: &str = "Library/Application Support/APK installer";
const CONFIG_FILE: &str = "config.toml";
const CONFIG_TEMPLATE: &str = r#"platforms = [ "quest", "pico" ]
app_ids = [ "com.company.product.app1", "com.company.product.app2" ]"#;

#[derive(Deserialize)]
pub struct Config {
	platforms: Vec<Platform>,
	app_ids: Vec<AppId>,
	version: Version,
}

impl Config {
	pub fn build(version: &str) -> Result<Config, Error> {
		let version = parse_version(version)?;
		let home_path =
			std::env::home_dir().ok_or(IOError(String::from("Failed to find home directory.")))?;
		let folder_path = home_path.join(CONFIG_PATH);
		let file_path = folder_path.join(CONFIG_FILE);
		let mut config = match fs::read_to_string(&file_path) {
			Ok(file) => file,
			Err(_) => {
				if !fs::exists(&folder_path)? {
					fs::create_dir_all(&folder_path)?;
				}
				let mut file = File::create(&file_path)?;
				file.write_all(CONFIG_TEMPLATE.as_bytes())?;
				let file_path_str = file_path.to_str().unwrap();
				println!(
					"Config file not found. Created one at {file_path_str}. Modify it and try again"
				);
				return Err(CommandError("Config file not found."));
			}
		};
        
        // Version is a command-line argument, not an entry on the config file.
		let version_value = format!("\nversion = \"{version}\"");
		config += &version_value;
		let config = toml::from_str(config.as_str())?;
		Ok(config)
	}
}

/// Parses an input string into a version string.
///
/// # Arguments
///
/// * `input`: the input string to be parsed.
///
/// returns: `Ok` if parsing was successful, `None` if not.
///
/// # Examples
///
/// ```
/// assert_eq!(apk_installer::parse_version("5.1"), Ok("5.1".to_string()));
/// assert_eq!(apk_installer::parse_version("5.1.0"), Ok("5.1.0".to_string()));
/// assert_eq!(apk_installer::parse_version("5.1.1.1"), Ok("5.1.1.1".to_string()));
/// ```
pub fn parse_version(input: &str) -> Result<String, Error> {
	let regex = Regex::new(r"\b\d+(\.\d+\b)+")?;
	let result = regex
		.find(input)
		.ok_or(Error::ParseError(String::from("Failed to parse version.")))?;
	Ok(String::from(result.as_str()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_valid_2_regions() {
		assert_eq!(parse_version("5.1"), Ok("5.1".to_string()));
		assert_eq!(parse_version("0.0"), Ok("0.0".to_string()));
		assert_eq!(parse_version("0.1"), Ok("0.1".to_string()));
		assert_eq!(parse_version("155.1"), Ok("155.1".to_string()));
		assert_eq!(parse_version("5.999"), Ok("5.999".to_string()));
		assert_eq!(parse_version("999.999"), Ok("999.999".to_string()));
	}

	#[test]
	fn parse_valid_3_regions() {
		assert_eq!(parse_version("5.1.0"), Ok("5.1.0".to_string()));
		assert_eq!(parse_version("0.0.0"), Ok("0.0.0".to_string()));
		assert_eq!(parse_version("0.1.0"), Ok("0.1.0".to_string()));
		assert_eq!(parse_version("1.155.1"), Ok("1.155.1".to_string()));
		assert_eq!(parse_version("5.0.999"), Ok("5.0.999".to_string()));
		assert_eq!(parse_version("999.999.999"), Ok("999.999.999".to_string()));
	}

	#[test]
	fn parse_valid_many_regions() {
		assert_eq!(parse_version("5.1.1.1"), Ok("5.1.1.1".to_string()));
		assert_eq!(parse_version("0.0.0.0.0.0"), Ok("0.0.0.0.0.0".to_string()));
		assert_eq!(
			parse_version("155.1.2.3.4.5"),
			Ok("155.1.2.3.4.5".to_string())
		);
		assert_eq!(
			parse_version("1.22.333.4444.55555.666666"),
			Ok("1.22.333.4444.55555.666666".to_string())
		);
	}

	#[test]
	fn parse_invalid() {
		// 1 region
		assert!(parse_version("1").is_err());
		assert!(parse_version("a").is_err());
		assert!(parse_version("").is_err());
		assert!(parse_version("1a").is_err());
		assert!(parse_version("a1").is_err());
		assert!(parse_version("aa").is_err());
		// 2 regions
		assert!(parse_version("1.a").is_err());
		assert!(parse_version("a.1").is_err());
		assert!(parse_version("1.").is_err());
		assert!(parse_version(".1").is_err());
		assert!(parse_version("1.1a").is_err());
		assert!(parse_version("1a.1").is_err());
		// other
		assert!(parse_version("1. 0").is_err());
		assert!(parse_version("1 .0").is_err());
		assert!(parse_version("1.a.0").is_err());
	}
}
