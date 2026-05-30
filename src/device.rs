use crate::config::Platform;
use crate::error::Error;
use crate::installation::CommandOutcome;
use crate::package::Package;
use regex::Regex;
use std::fmt::Display;
use std::process::Command;

pub struct Device {
	name: String,
	id: String,
	platform: String,
}

impl Device {
	pub fn supports(&self, package: &Package) -> bool {
		package
			.platforms()
			.iter()
			.any(|p| match package.match_file_name() {
				false => &self.platform == p,
				true => package.file_name().to_lowercase().contains(&self.platform),
			})
	}

	pub async fn install(&self, package: &Package) -> CommandOutcome {
		let path = package.path();
		let command = tokio::process::Command::new("adb")
			.args(["-s", &self.id, "install", path])
			.output();
		let description = format!("Installation of {} on {}", package.id(), self.name);
		match command.await {
			Ok(output) if output.status.success() => CommandOutcome::from_success(&description),
			Ok(output) => {
				let error = Error::from_installation_error(&output.stderr);
				CommandOutcome::from_error(&description, error)
			}
			Err(e) => CommandOutcome::from_error(&description, Error::Installation(e.to_string())),
		}
	}

	pub async fn uninstall(&self, package: &Package) -> CommandOutcome {
		let description = format!("Uninstallation of {} on {}", package.id(), self.name);
		let command = tokio::process::Command::new("adb")
			.args(["-s", &self.id, "uninstall", package.id()])
			.output();
		match command.await {
			Ok(output) if output.status.success() => CommandOutcome::from_success(&description),
			Ok(output) => {
				let message = String::from_utf8_lossy(&output.stdout);
				let error = Error::Uninstall(message.to_string());
				CommandOutcome::from_error(&description, error)
			}
			Err(e) => {
				let error = Error::Uninstall(e.to_string());
				CommandOutcome::from_error(&description, error)
			}
		}
	}

	pub async fn has_app_installed(&self, app_name: &str) -> Result<bool, Error> {
		let output = tokio::process::Command::new("adb")
			.args([
				"-s",
				&self.id,
				"shell",
				"pm list packages",
				"| grep",
				app_name,
			])
			.output();
		match output.await {
			Ok(output) => Ok(output.status.success()),
			Err(e) => Err(Error::Uninstall(e.to_string())),
		}
	}

	fn from_str_with_platforms(line: &str, platforms: &[Platform]) -> Option<Device> {
		let (id, platform) = Self::parse_info(line, platforms)?;
		let name = Self::parse_name(&id).ok()?;
		let device = Self { name, id, platform };
		Some(device)
	}

	fn parse_name(id: &str) -> Result<String, Error> {
		let output = Command::new("adb")
			.args(["-s", id, "shell", "settings get global device_name"])
			.output()?;
		let name = String::from_utf8(output.stdout)?;
		match name.len() {
			0 => Err(Error::NoDeviceName),
			_ => Ok(String::from(name.trim_end())),
		}
	}

	fn parse_info(line: &str, platforms: &[Platform]) -> Option<(String, Platform)> {
		let regex = Regex::new(r"(\w+)\s+.*model:(\w+)\sdevice:(\w+)").ok()?;
		let caps = regex.captures(line)?;
		let id = String::from(caps.get(1)?.as_str());
		let model = caps.get(2)?.as_str();
		let device_name = caps.get(3)?.as_str();
		let platform = get_platform(model, platforms).or(get_platform(device_name, platforms))?;
		Some((id, platform))
	}

	pub fn get_devices(platforms: &[Platform]) -> Result<Vec<Device>, Error> {
		let output = Command::new("adb").args(["devices", "-l"]).output()?;
		let output = String::from_utf8(output.stdout)?;
		let header_line_ix = output
			.lines()
			.position(|l| l.contains("List of devices attached"))
			.ok_or(Error::DevicesFetching)?;

		let devices = output
			.lines()
			.skip(header_line_ix + 1)
			.filter(|l| !l.is_empty())
			.filter_map(|l| Self::from_str_with_platforms(l, platforms))
			.collect();
		Ok(devices)
	}
}

impl Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}] {} ({})", self.platform, self.name, self.id)
	}
}

fn get_platform(identifier: &str, platforms: &[Platform]) -> Option<Platform> {
	let identifier = identifier.to_lowercase();
	let platform = platforms.iter().find(|p| identifier.contains(*p))?;
	Some(platform.clone())
}

#[cfg(test)]
mod tests {
	use super::*;

	static PLATFORMS: [&str; 3] = ["pico", "quest", "sm_g"];

	fn get_platforms() -> [String; 3] {
		PLATFORMS.map(String::from)
	}

	#[test]
	fn test_get_platform_pico() {
		let pico = Some(String::from("pico"));
		let platforms = get_platforms();
		assert_eq!(get_platform("Pico_Neo_3", &platforms), pico);
		assert_eq!(get_platform("PICOA7H10", &platforms), pico);
		assert_eq!(get_platform("PICOA8110", &platforms), pico);
	}

	#[test]
	fn test_get_platform_quest() {
		let pico = Some(String::from("quest"));
		let platforms = get_platforms();
		assert_eq!(get_platform("Quest_2", &platforms), pico);
		assert_eq!(get_platform("Quest_3", &platforms), pico);
		assert_eq!(get_platform("Quest_3S", &platforms), pico);
		assert_eq!(get_platform("Quest_3_2", &platforms), pico);
	}

	#[test]
	fn test_get_platform_galaxy() {
		let pico = Some(String::from("sm_g"));
		let platforms = get_platforms();
		assert_eq!(get_platform("SM_G950F", &platforms), pico);
	}

	#[test]
	fn parse_pico_neo_3() {
		let data = "PA7L50MGF8290021W      device usb:34873344X product:A7H10 model:Pico_Neo_3 \
		 device:PICOA7H10 transport_id:2";
		let platforms = get_platforms();
		let info = Device::parse_info(data, &platforms);
		assert!(info.is_some());
		let (id, platform) = info.unwrap();
		assert_eq!(id, String::from("PA7L50MGF8290021W"));
		assert_eq!(platform, "pico");
	}

	#[test]
	fn parse_pico_neo_4() {
		let data = "PA8150MGGB230744G      device usb:34603008X product:Phoenix_ovs model:A8110 \
		 device:PICOA8110 transport_id:7";
		let platforms = get_platforms();
		let info = Device::parse_info(data, &platforms);
		assert!(info.is_some());
		let (id, platform) = info.unwrap();
		assert_eq!(id, String::from("PA8150MGGB230744G"));
		assert_eq!(platform, "pico");
	}

	#[test]
	fn parse_pico_quest_2() {
		let data = "1WMHHA63PR1501         device usb:34603008X product:hollywood model:Quest_2 \
		 device:hollywood transport_id:9";
		let platforms = get_platforms();
		let info = Device::parse_info(data, &platforms);
		assert!(info.is_some());
		let (id, platform) = info.unwrap();
		assert_eq!(id, String::from("1WMHHA63PR1501"));
		assert_eq!(platform, "quest");
	}

	#[test]
	fn parse_pico_galaxy_s8() {
		let data = "ce031713396bc92803     device usb:1048576X product:dreamltexx model:SM_G950F \
		 device:dreamlte transport_id:4";
		let platforms = get_platforms();
		let info = Device::parse_info(data, &platforms);
		assert!(info.is_some());
		let (id, platform) = info.unwrap();
		assert_eq!(id, String::from("ce031713396bc92803"));
		assert_eq!(platform, "sm_g");
	}
}
