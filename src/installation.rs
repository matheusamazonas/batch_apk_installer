use crate::device::Device;
use crate::error::Error;
use crate::package::Package;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

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
		let mut handles = vec![];
		let outcomes = vec![];
		let outcomes = Arc::new(Mutex::new(outcomes));
		
		for package in self.packages {
			let package = package.clone();
			let device = self.device.clone();
			let outcomes = outcomes.clone();
			let handle = thread::spawn(move || {
				let outcome = perform_install(&device, &package);
				match outcomes.lock() {
				    Ok(mut outcomes) => outcomes.push(outcome),
					Err(e) => eprintln!("Failed to acquire lock when performing install: {:?}", e),
				}
			});
			handles.push(handle);
		}
		
		for handle in handles {
			handle.join().unwrap();
		}
		
		// It's fine to unwrap here because all threads have joined.
		outcomes.lock().unwrap().iter().map(clone_outcome).collect()
	}
}

fn clone_outcome(r: &Result<String, Error>) -> Result<String, Error> {
	match r {
		Ok(o) => Ok(o.clone()),
		Err(e) => Err(e.clone())
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
