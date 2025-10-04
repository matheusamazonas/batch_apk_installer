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
			let matches = packages.iter().filter(|p| is_package_match(device, p));
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
		let device = self.device;
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
		write!(
			f,
			"Installation of {} ({}) on {}",
			self.package.id(),
			self.package.file_name(),
			self.device.name()
		)
	}
}

fn is_package_match(device: &Device, package: &Package) -> bool {
	package.platforms().iter()
		.any(|p| match package.match_file_name() {
			false => device.platform() == p,
			true => package
				.file_name()
				.to_lowercase()
				.contains(device.platform()),
		})
}
