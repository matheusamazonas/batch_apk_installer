use regex::{self, Regex};
use std::io;
use std::process::{Command, Stdio};

fn find_adb() -> bool {
	Command::new("adb")
		.args(["--version"])
		.stdout(Stdio::null())
		.status()
		.is_ok()
}

fn get_version() -> Option<String> {
	let regex = Regex::new(r"\b\d+\.\d+\b").ok()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input).ok()?;
	let m = regex.find(&input)?;
	Some(String::from(m.as_str()))
}

fn main() {
	if !find_adb() {
		println!("ADB not found. Please ensure that ADB is installed.");
		return;
	}

	let version = loop {
		println!("Which version would you like to install?");
		match get_version() {
			Some(version) => break version,
			None => print!("Wrong version format. Example: 5.1. "),
		}
	};

	println!("Version: {version:?}");
}
