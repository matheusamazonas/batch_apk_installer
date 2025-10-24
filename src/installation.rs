use crate::device::Device;
use crate::error::Error;
use crate::error::Error::Uninstall;
use crate::package::Package;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct DeviceInstallations {
	device: Arc<Device>,
	packages: Vec<Arc<Package>>,
	uninstall_first: bool,
}

pub struct CommandOutcome {
	description: String,
	error: Option<Error>,
}

impl CommandOutcome {
	pub fn description(&self) -> &String {
		&self.description
	}

	pub fn error(&self) -> Option<&Error> {
		self.error.as_ref()
	}
}

impl DeviceInstallations {
	pub fn build_requests(
		devices: &[Arc<Device>], packages: &[Arc<Package>], uninstall_first: bool,
	) -> Vec<DeviceInstallations> {
		let mut requests: Vec<DeviceInstallations> = Vec::new();
		for device in devices {
			let matches = packages.iter().filter(|p| is_package_match(device, p));
			let mut packages = vec![];
			for package in matches {
				let package = package.clone();
				packages.push(package);
			}
			let device = device.clone();
			requests.push(DeviceInstallations { device, packages, uninstall_first});
		}
		requests
	}

	pub fn count(&self) -> usize {
		self.packages.len()
	}

	pub fn perform(self) -> ReceiverStream<CommandOutcome> {
		let (tx, rx) = mpsc::channel(self.packages.len());
		for package in self.packages {
			let device = self.device.clone();
			let tx = tx.clone();
			tokio::task::spawn(async move {
				if self.uninstall_first {
					let uninstall_outcome = perform_uninstall(&device, &package).await;
					tx.send(uninstall_outcome)
						.await
						.expect("Error sending operation outcome.");
				}
				let install_outcome = perform_install(&device, &package).await;
				tx.send(install_outcome)
					.await
					.expect("Error sending operation outcome.");
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

fn parse_installation_error(error: &[u8]) -> Error {
	let error = String::from_utf8_lossy(error);
	if error.contains("INSTALL_FAILED_UPDATE_INCOMPATIBLE") {
		Error::PackageSignatureMismatch
	} else if error.contains("INSTALL_FAILED_VERSION_DOWNGRADE") {
		Error::PackageDowngrade
	} else {
		Error::Installation(String::from(error))
	}
}

async fn perform_uninstall(device: &Device, package: &Package) -> CommandOutcome {
	let description = format!("Uninstallation of {} on {}", package.id(), device.name());
	let output = tokio::process::Command::new("adb")
		.args(["-s", device.id(), "uninstall", package.id()])
		.output();
	match output.await {
		Ok(output) if output.status.success() => CommandOutcome {
			description,
			error: None,
		},
		Ok(output) => {
			let message = String::from_utf8_lossy(&output.stderr);
			CommandOutcome {
				description,
				error: Some(Uninstall(message.to_string())),
			}
		}
		Err(e) => CommandOutcome {
			description,
			error: Some(Uninstall(e.to_string())),
		},
	}
}

async fn perform_install(device: &Device, package: &Package) -> CommandOutcome {
	let path = package.path();
	let output = tokio::process::Command::new("adb")
		.args(["-s", device.id(), "install", path])
		.output();
	let error = match output.await {
		Ok(output) if output.stderr.is_empty() => None, // Success
		Ok(output) => {
			let error = parse_installation_error(&output.stderr);
			Some(error)
		}
		Err(e) => Some(Error::Installation(e.to_string())),
	};
	let description = format!("Installation of {} on {}", package.id(), device.name());
	CommandOutcome { description, error }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_signature_mismatch_error() {
		let error = "adb: failed to install airhockey.apk: Failure \
		[INSTALL_FAILED_UPDATE_INCOMPATIBLE: Package com.lazysquirrellabs.airhockey signatures \
		do not match previously installed version; ignoring!]";
		assert_eq!(
			parse_installation_error(error.as_bytes()),
			Error::PackageSignatureMismatch
		);
	}

	#[test]
	fn parse_downgrade_error() {
		let error = "adb: failed to install 5.1/CorpusVR-5.1-5010001-Dashboard.apk: \
		Failure [INSTALL_FAILED_VERSION_DOWNGRADE]";
		assert_eq!(
			parse_installation_error(error.as_bytes()),
			Error::PackageDowngrade
		);
	}

	#[test]
	fn parse_generic_error() {
		// Sometimes ADB throws errors without no content, just a header, like the one below.
		let error = "adb: failed to install 5.1/CorpusVR-5.1-5010001-Dashboard.apk:\n";
		assert_eq!(
			parse_installation_error(error.as_bytes()),
			Error::Installation(String::from(error))
		);
	}
}
