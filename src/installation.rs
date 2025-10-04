use crate::device::Device;
use crate::error::Error;
use crate::package::Package;
use std::process::Command;

pub struct DeviceInstallations {
	device: Device,
	packages: Vec<Package>,
}

impl DeviceInstallations {
	pub fn build_requests(
		devices: &Vec<Device>,
		packages: &Vec<Package>,
	) -> Vec<DeviceInstallations> {
		let mut requests: Vec<DeviceInstallations> = Vec::new();
		for device in devices {
			let device = device.clone();
			let matches = packages.iter()
				.filter(|p| is_package_match(&device, p));
			let mut packages = vec![];
			for package in matches {
				let package = package.clone();
				packages.push(package);
			}
			let request = DeviceInstallations { device, packages };
			requests.push(request);
		}
		requests
	}
	pub fn perform(self) -> Vec<Result<String, Error>> {
		self.packages.iter()
			.map(|p| perform_install(&self.device, p))
			.collect()
	}
}

fn is_package_match(device: &Device, package: &Package) -> bool {
	package.platforms()
		.iter()
		.any(|p| match package.match_file_name() {
			false => device.platform() == p,
			true => package
				.file_name()
				.to_lowercase()
				.contains(device.platform()),
		})
}

fn perform_install(device: &Device, package: &Package) -> Result<String, Error> {
	let path = package.path();
	let output = Command::new("adb")
		.args(["-s", device.id(), "install", path])
		.output();
	match output {
		Ok(_) => {
			let message = format!("Successfully installed {} ({}) on {}. ✅", 
								  package.id(), package.file_name(), device.name());
			Ok(message)
		},
		Err(e) => {
			let message = format!("Failed to install {} ({}) on {}. ❌ Error: {}",
			                       package.id(), package.file_name(), device.name(), e);
			Err(Error::Installation(message))
		},
	}
}
