use crate::device::Device;
use crate::error::Error;
use crate::package::Package;
use std::fmt::Display;
use std::process::Command;

pub struct InstallationRequest<'a> {
	device: &'a Device,
	package: &'a Package,
}

impl InstallationRequest<'_> {
	pub fn build_requests<'a>(
		devices: &'a [Device],
		packages: &'a [Package],
	) -> Vec<InstallationRequest<'a>> {
		let mut requests: Vec<InstallationRequest> = Vec::new();
		for device in devices {
			let matches = packages
				.iter()
				.filter(|p| p.platforms().contains(device.platform()));
			for package in matches {
				let request = InstallationRequest { device, package };
				requests.push(request);
			}
		}
		requests
	}
	pub fn perform(self) -> Result<(), Error> {
		let package = self.package;
		let path = package.path();
		let package_id = package.id();
		let device = self.device;
		let device_name = device.name();
		println!("Installing {package_id} on {device_name}");
		let output = Command::new("adb")
			.args(["-s", device.id(), "install", path])
			.output();
		match output {
			Ok(_) => Ok(()),
			Err(e) => Err(Error::Installation(format!("{}", e))),
		}
	}
}

impl Display for InstallationRequest<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Install of {} on {}", self.package.id(), self.device.name())
	}
}
