use regex::{self, Regex};
use std::io;
use std::process::{Command, Stdio};

pub struct Config {
	version: String,
}

pub fn find_adb() -> bool {
	Command::new("adb")
		.args(["--version"])
		.stdout(Stdio::null())
		.status()
		.is_ok()
}

fn parse_version(input: &str) -> Option<String> {
	let regex = Regex::new(r"\b\d+\.\d+\b").ok()?;
	let result = regex.find(&input)?;
	Some(String::from(result.as_str()))
}

pub fn get_version() -> Option<String> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).ok()?;
	parse_version(&input)
}
