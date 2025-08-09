use regex::{self, Regex};

pub mod adb;

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
/// assert_eq!(adb_installer::parse_version("5.1"), Some("5.1".to_string()));
/// assert_eq!(adb_installer::parse_version("5.1.0"), Some("5.1.0".to_string()));
/// assert_eq!(adb_installer::parse_version("5.1.1.1"), Some("5.1.1.1".to_string()));
/// ```
pub fn parse_version(input: &str) -> Option<String> {
	let regex = Regex::new(r"\b\d+(\.\d+\b)+").ok()?;
	let result = regex.find(&input)?;
	Some(String::from(result.as_str()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_valid_2_regions() {
		assert_eq!(parse_version("5.1"), Some("5.1".to_string()));
		assert_eq!(parse_version("0.0"), Some("0.0".to_string()));
		assert_eq!(parse_version("0.1"), Some("0.1".to_string()));
		assert_eq!(parse_version("155.1"), Some("155.1".to_string()));
		assert_eq!(parse_version("5.999"), Some("5.999".to_string()));
		assert_eq!(parse_version("999.999"), Some("999.999".to_string()));
	}

	#[test]
	fn parse_valid_3_regions() {
		assert_eq!(parse_version("5.1.0"), Some("5.1.0".to_string()));
		assert_eq!(parse_version("0.0.0"), Some("0.0.0".to_string()));
		assert_eq!(parse_version("0.1.0"), Some("0.1.0".to_string()));
		assert_eq!(parse_version("1.155.1"), Some("1.155.1".to_string()));
		assert_eq!(parse_version("5.0.999"), Some("5.0.999".to_string()));
		assert_eq!(
			parse_version("999.999.999"),
			Some("999.999.999".to_string())
		);
	}

	#[test]
	fn parse_valid_many_regions() {
		assert_eq!(parse_version("5.1.1.1"), Some("5.1.1.1".to_string()));
		assert_eq!(
			parse_version("0.0.0.0.0.0"),
			Some("0.0.0.0.0.0".to_string())
		);
		assert_eq!(
			parse_version("155.1.2.3.4.5"),
			Some("155.1.2.3.4.5".to_string())
		);
		assert_eq!(
			parse_version("1.22.333.4444.55555.666666"),
			Some("1.22.333.4444.55555.666666".to_string())
		);
	}

	#[test]
	fn parse_invalid() {
		// 1 region
		assert_eq!(parse_version("1"), None);
		assert_eq!(parse_version("a"), None);
		assert_eq!(parse_version(""), None);
		assert_eq!(parse_version("1a"), None);
		assert_eq!(parse_version("a1"), None);
		assert_eq!(parse_version("aa"), None);
		// 2 regions
		assert_eq!(parse_version("1.a"), None);
		assert_eq!(parse_version("a.1"), None);
		assert_eq!(parse_version("1."), None);
		assert_eq!(parse_version(".1"), None);
		assert_eq!(parse_version("1.1a"), None);
		assert_eq!(parse_version("1a.1"), None);
		// other
		assert_eq!(parse_version("1. 0"), None);
		assert_eq!(parse_version("1 .0"), None);
		assert_eq!(parse_version("1.a.0"), None);
	}
}
