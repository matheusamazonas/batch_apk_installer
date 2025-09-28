use crate::config::Platform;
use crate::error::Error;
use regex::Regex;
use std::fmt::Display;
use std::process::Command;

pub struct Device {
	pub name: String,
	id: String,
	platform: String,
}

impl Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}] {} ({})", self.platform, self.name, self.id)
	}
}

pub fn get_devices(platforms: &[Platform]) -> Result<Vec<Device>, Error> {
	let output = Command::new("adb").args(["devices", "-l"]).output()?;
	let output = String::from_utf8(output.stdout)?;
	let header_line_ix = output
		.lines()
		.position(|l| l.contains("List of devices attached"))
		.ok_or(Error::Device(String::from(
			"Failed to fetch Android devices.",
		)))?;

	let devices = output
		.lines()
		.skip(header_line_ix + 1)
		.filter(|l| !l.is_empty())
		.filter_map(|l| parse_device(l, platforms))
		.collect();
	Ok(devices)
}

fn parse_device(line: &str, platforms: &[Platform]) -> Option<Device> {
	let (id, platform) = parse_device_info(line, platforms)?;
	let name = get_device_name(&id).ok()?;
	let device = Device { id, name, platform };
	Some(device)
}

fn parse_device_info(line: &str, platforms: &[Platform]) -> Option<(String, Platform)> {
	let regex = Regex::new(r"(\w+)\s+.*model:(\w+)\sdevice:(\w+)").ok()?;
	let caps = regex.captures(line)?;
	let id = String::from(caps.get(1)?.as_str());
	let model = caps.get(2)?.as_str();
	let device_name = caps.get(3)?.as_str();
	let platform = get_platform(model, platforms)
		.or(get_platform(device_name, platforms))?;
	Some((id, platform))
}

fn get_platform(identifier: &str, platforms: &[Platform]) -> Option<Platform> {
	let identifier = identifier.to_lowercase();
	let platform = platforms.iter().find(|p| identifier.contains(*p))?;
	Some(platform.clone())
}

fn get_device_name(id: &str) -> Result<String, Error> {
	let output = Command::new("adb")
		.args(["-s", id, "shell", "settings get global device_name"])
		.output()?;
	let name = String::from_utf8(output.stdout)?;
	match name.len() {
		0 => Err(Error::Device(String::from("No device name provided."))),
		_ => Ok(String::from(name.trim_end())),
	}
}
