use crate::error::Error;
use std::process::{Command, Stdio};

pub fn has_adb() -> bool {
	check_command("adb", "--version")
}

pub fn has_aapt() -> bool {
    check_command("aapt2", "version")
}

fn check_command(command: &str, args: &str) -> bool {
    Command::new(command)
        .args([args])
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
		.ok_or(Error::CommandError("Failed to fetch Android devices."))?;

	let devices: Vec<String> = output_str
		.lines()
		.skip(header_line_ix + 1)
		.filter_map(parse_device)
		.collect();
	return Ok(devices);

	fn parse_device(line: &str) -> Option<String> {
		if !line.is_empty() {
			Some(String::from(line))
		} else {
			None
		}
	}
}
