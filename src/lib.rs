use regex::{self, Regex};
use error::Error;

pub mod android;
pub mod error;

type Platform = String;
type App = String;
type Version = String;

pub struct Config {
    platform: Platform,
    app: App,
    version: Version,
}

impl Config {
    pub  fn build(version: &str) -> Result<Config, Error> {
        let version = parse_version(version)?;
        Err(Error::AdbNotFound)
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
	let result = regex.find(&input).ok_or(Error::ParseError)?;
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
		assert_eq!(
			parse_version("999.999.999"),
            Ok("999.999.999".to_string())
		);
	}

	#[test]
	fn parse_valid_many_regions() {
		assert_eq!(parse_version("5.1.1.1"), Ok("5.1.1.1".to_string()));
		assert_eq!(
			parse_version("0.0.0.0.0.0"),
            Ok("0.0.0.0.0.0".to_string())
		);
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
		assert_eq!(parse_version("1"), Err(Error::ParseError));
		assert_eq!(parse_version("a"), Err(Error::ParseError));
		assert_eq!(parse_version(""), Err(Error::ParseError));
		assert_eq!(parse_version("1a"), Err(Error::ParseError));
		assert_eq!(parse_version("a1"), Err(Error::ParseError));
		assert_eq!(parse_version("aa"), Err(Error::ParseError));
		// 2 regions
		assert_eq!(parse_version("1.a"), Err(Error::ParseError));
		assert_eq!(parse_version("a.1"), Err(Error::ParseError));
		assert_eq!(parse_version("1."), Err(Error::ParseError));
		assert_eq!(parse_version(".1"), Err(Error::ParseError));
		assert_eq!(parse_version("1.1a"), Err(Error::ParseError));
		assert_eq!(parse_version("1a.1"), Err(Error::ParseError));
		// other
		assert_eq!(parse_version("1. 0"), Err(Error::ParseError));
		assert_eq!(parse_version("1 .0"), Err(Error::ParseError));
		assert_eq!(parse_version("1.a.0"), Err(Error::ParseError));
	}
}
