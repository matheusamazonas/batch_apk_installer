use crate::device::Device;
use crate::error::Error;
use crate::package::Package;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct DeviceInstallations {
	device: Arc<Device>,
	packages: Vec<Arc<Package>>,
}

impl DeviceInstallations {
	pub fn build_requests(
		devices: &[Arc<Device>], packages: &[Arc<Package>],
	) -> Vec<DeviceInstallations> {
		let mut requests: Vec<DeviceInstallations> = Vec::new();
		for device in devices {
			let matches = packages.iter().filter(|p| is_package_match(&device, p));
			let mut packages = vec![];
			for package in matches {
				let package = package.clone();
				packages.push(package);
			}
			let device = device.clone();
			let request = DeviceInstallations { device, packages };
			requests.push(request);
		}
		requests
	}

	pub fn count(&self) -> usize {
		self.packages.len()
	}

	pub fn perform(self) -> ReceiverStream<Result<String, Error>> {
		let (tx, rx) = mpsc::channel(self.packages.len());
		for package in self.packages {
			let device = self.device.clone();
			let tx = tx.clone();
			tokio::task::spawn(async move {
				let outcome = perform_install(&device, &package).await;
				tx.send(outcome).await.expect("Error sending outcome.");
			});
		}

		ReceiverStream::new(rx)
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

async fn perform_install(device: &Device, package: &Package) -> Result<String, Error> {
	let path = package.path();
	let output = tokio::process::Command::new("adb")
		.args(["-s", device.id(), "install", path])
		.output();
	match output.await {
		Ok(output) if output.stderr.is_empty() => {
			let message = format!(
				"Successfully installed {} ({}) on {}. ✅",
				package.id(),
				package.file_name(),
				device.name()
			);
			Ok(message)
		}
		Ok(output) => {
			let error = String::from_utf8_lossy(&output.stderr);
			let mut message = format!(
				"Failed to install {} ({}) on {}. ❌ ",
				package.id(),
				package.file_name(),
				device.name()
			);
			let body = if error.contains("INSTALL_FAILED_UPDATE_INCOMPATIBLE") {
				String::from("Signatures don't match.")
			} else {
				format!("Error: {error}")
			};
			message.push_str(&body);
			Err(Error::Installation(message))
		}
		Err(e) => {
			let message = format!(
				"Failed to install {} ({}) on {}. ❌ Error: {}",
				package.id(),
				package.file_name(),
				device.name(),
				e
			);
			Err(Error::Installation(message))
		}
	}
}
