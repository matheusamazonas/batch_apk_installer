use std::io;
use std::process::{Command, Stdio};
use std::string;

#[derive(Debug)]
pub enum Error {
	CommandError,
	ParseError,
}

impl From<io::Error> for Error {
	fn from(_: io::Error) -> Self {
		Error::CommandError
	}
}

impl From<string::FromUtf8Error> for Error {
	fn from(_: string::FromUtf8Error) -> Self {
		Error::ParseError
	}
}

pub fn is_adb_present() -> bool {
	Command::new("adb")
		.args(["--version"])
		.stdout(Stdio::null())
		.status()
		.is_ok()
}

pub fn get_devices() -> Result<Vec<String>, Error> {
	let output = Command::new("adb").args(["devices", "-l"]).output()?;
	let output_str = String::from_utf8(output.stdout)?;
	let header_line_ix = output_str
		.lines()
		.position(|l| l.contains("List of devices attached"))
		.ok_or(Error::CommandError)?;

	let devices: Vec<String> = output_str
		.lines()
		.skip(header_line_ix + 1)
		.filter_map(parse_device)
		.collect();
	return Ok(devices);

	fn parse_device(line: &str) -> Option<String> {
		if line.len() > 0 {
			Some(String::from(line))
		} else {
			None
		}
	}
}
