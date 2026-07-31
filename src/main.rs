use crate::config::Config;
use crate::console::Command;
use crate::device::Device;
use crate::installation::DeviceInstallations;
use crate::package::Package;
use futures::{StreamExt, stream};
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

mod config;
mod console;
mod device;
mod error;
mod installation;
mod package;

#[tokio::main]
async fn main() {
	let (packages_folder, uninstall) = match console::get_command() {
		Ok(Command::Install { folder, uninstall }) => (folder, uninstall),
		Ok(Command::Help) => {
			console::print_help();
			process::exit(0);
		}
		Err(e) => {
			console::print_error(&e.to_string());
			process::exit(1);
		}
	};

	let config = match Config::build() {
		Ok(config) => config,
		Err(e) => {
			let message = format!("Error when loading config: {e}.");
			console::print_error(&message);
			process::exit(1)
		}
	};

	let devices: Vec<_> = match Device::get_devices(config.platforms()) {
		Ok(devices) if !devices.is_empty() => devices.into_iter().map(Arc::new).collect(),
		Ok(_) => {
			console::print_error("No devices were found.");
			process::exit(1)
		}
		Err(e) => {
			let message = format!("Error when fetching devices: {e}.");
			console::print_error(&message);
			process::exit(1)
		}
	};

	for device in &devices {
		println!("Found device: {device}.");
	}

	let packages_dir = PathBuf::from(config.directory()).join(packages_folder);
	let packages: Vec<_> = match Package::find_all(&packages_dir, config.packages()) {
		Ok(packages) => packages.map(Arc::new).collect(),
		Err(e) => {
			console::print_error(&e.to_string());
			process::exit(1);
		}
	};

	if packages.is_empty() {
		console::print_error("No packages found.");
		process::exit(1);
	}

	let installs = DeviceInstallations::build_requests(&devices, &packages, uninstall);
	match installs.len() {
		0 => {
			console::print_error("No installation requests found.");
			process::exit(1);
		}
		device_count => {
			let total_installs = installs.iter().fold(0, |acc, e| acc + e.count());
			println!("Running {total_installs} installation(s) on {device_count} device(s)...");
			let streams = installs.into_iter().map(DeviceInstallations::perform);
			let mut stream = stream::select_all(streams);
			while let Some(outcome) = stream.next().await {
				let description = outcome.description();
				match outcome.error() {
					Some(e) => {
						let error = format!("{description} failed: {e}.");
						console::print_error(&error);
					}
					None => println!("\x1b[92m{description} completed successfully.\x1b[0m"),
				}
			}
		}
	}
}
