use crate::error::Error;
use std::fs;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct PackageFile {
	path: String,
	id: String,
}

pub fn has_adb() -> bool {
	check_command("adb", "--version")
}

pub fn has_aapt() -> bool {
	check_command("aapt2", "version")
}

pub fn find_package_files(dir: &str) -> Result<Vec<PackageFile>, Error> {
	let files = find_apk_files(dir)?
		.into_iter()
		.filter_map(|f| get_package_file(&f).ok())
		.collect();
	Ok(files)
}

fn check_command(command: &str, args: &str) -> bool {
	Command::new(command)
		.args([args])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.status()
		.is_ok()
}

fn find_apk_files(dir: &str) -> Result<Vec<String>, Error> {
	let entries = fs::read_dir(dir)?
		.filter_map(|e| e.ok())
		.map(|e| e.path())
		.filter(|path| path.extension().is_some_and(|ext| ext == "apk"))
		.filter_map(|path| path.into_os_string().into_string().ok())
		.collect();
	Ok(entries)
}

fn get_package_file(path: &str) -> Result<PackageFile, Error> {
	let output = Command::new("aapt2")
		.args(["dump", "packagename", path])
		.output()?;
	if output.status.success() {
		let output = String::from_utf8(output.stdout)?;
		let id = String::from(output.trim_end());
		let path = String::from(path);
		let package = PackageFile { path, id };
		Ok(package)
	} else {
		Err(Error::CommandError("Failed to get APK package name."))
	}
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
