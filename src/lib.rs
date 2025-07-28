use regex::{self, Regex};
use std::process::{Command, Stdio};

pub fn find_adb() -> bool {
	Command::new("adb")
		.args(["--version"])
		.stdout(Stdio::null())
		.status()
		.is_ok()
}

pub fn parse_version(input: &str) -> Option<String> {
	let regex = Regex::new(r"\b\d+\.\d+\b").ok()?;
	let result = regex.find(&input)?;
	Some(String::from(result.as_str()))
}
