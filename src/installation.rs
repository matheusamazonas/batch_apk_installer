use crate::device::Device;
use crate::error::Error;
use crate::package::Package;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

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

	pub fn from_success(description: &str) -> CommandOutcome {
		CommandOutcome {
			description: String::from(description),
			error: None,
		}
	}

	pub fn from_error(description: &str, error: Error) -> CommandOutcome {
		CommandOutcome {
			description: String::from(description),
			error: Some(error),
		}
	}
}

pub struct DeviceInstallations {
	device: Arc<Device>,
	packages: Vec<Arc<Package>>,
	uninstall_first: bool,
}

impl DeviceInstallations {
	pub fn build_requests(
		devices: &[Arc<Device>], packages: &[Arc<Package>], uninstall_first: bool,
	) -> Vec<DeviceInstallations> {
		let mut requests: Vec<DeviceInstallations> = Vec::new();
		for device in devices {
			let matches: Vec<_> = packages.iter().filter(|p| device.supports(p)).collect();
			if matches.is_empty() {
				continue;
			}
			let mut packages = Vec::with_capacity(matches.len());
			for package in matches {
				let package = package.clone();
				packages.push(package);
			}
			let installations = DeviceInstallations {
				device: device.clone(),
				packages,
				uninstall_first,
			};
			requests.push(installations);
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
					match device.has_app_installed(package.id()).await {
						Ok(true) => {
							// App is installed, uninstall it first.
							let uninstall_outcome = device.uninstall(&package).await;
							tx.send(uninstall_outcome)
								.await
								.expect("Error sending uninstallation outcome.");
						}
						Ok(false) => (), // Do nothing because the app is not installed.
						Err(e) => {
							let error = CommandOutcome::from_error("Uninstall check failed", e);
							tx.send(error)
								.await
								.expect("Error sending uninstallation check error.");
						}
					}
				}
				let install_outcome = device.install(&package).await;
				tx.send(install_outcome)
					.await
					.expect("Error sending installation outcome.");
			});
		}
		ReceiverStream::new(rx)
	}
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
			Error::from_installation_error(error.as_bytes()),
			Error::PackageSignatureMismatch
		);
	}

	#[test]
	fn parse_downgrade_error() {
		let error = "adb: failed to install 5.1/CorpusVR-5.1-5010001-Dashboard.apk: \
		Failure [INSTALL_FAILED_VERSION_DOWNGRADE]";
		assert_eq!(
			Error::from_installation_error(error.as_bytes()),
			Error::PackageDowngrade
		);
	}

	#[test]
	fn parse_generic_error() {
		// Sometimes ADB throws errors without no content, just a header, like the one below.
		let error = "adb: failed to install 5.1/CorpusVR-5.1-5010001-Dashboard.apk:\n";
		assert_eq!(
			Error::from_installation_error(error.as_bytes()),
			Error::Installation(String::from(error))
		);
	}
}
