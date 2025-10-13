use crate::config::Config;
use crate::error::Error;
use crate::installation::DeviceInstallations;
use crate::package::PackageFile;
use futures::{StreamExt, stream};
use std::env;
use std::path::PathBuf;
use std::process;
use std::process::{Command, Stdio};
use std::sync::Arc;

mod config;
mod device;
mod error;
mod installation;
mod package;

pub fn has_adb() -> bool {
	command_exists("adb", "--version")
}

pub fn has_aapt() -> bool {
	command_exists("aapt2", "version")
}

fn command_exists(command: &str, args: &str) -> bool {
	Command::new(command)
		.args([args])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.status()
		.is_ok()
}

#[tokio::main]
async fn main() {
	if !has_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	if !has_aapt() {
		eprintln!("AAPT not found. Please ensure that AAPT is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	let Some(version) = args.get(1) else {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	};

	let config = match Config::build(&args[1]) {
		Ok(config) => config,
		Err(Error::Config(e)) => {
			eprintln!("Config error: {e}.");
			process::exit(1);
		}
		Err(e) => {
			eprintln!("Error when loading config: {e:?}");
			process::exit(1)
		}
	};

	let devices: Vec<_> = match device::get_devices(config.platforms()) {
		Ok(devices) if !devices.is_empty() => devices.into_iter().map(Arc::new).collect(),
		Ok(_) => {
			eprintln!("No devices were found.");
			process::exit(1)
		}
		Err(e) => {
			eprintln!("Error when fetching devices: {e:?}");
			process::exit(1)
		}
	};

	for device in &devices {
		println!("Found device: {device}.");
	}

	let packages_dir = PathBuf::from(config.directory()).join(version);
	let packages: Vec<_> = match PackageFile::find_all(&packages_dir, config.packages()) {
		Ok(packages) => packages.into_iter().map(Arc::new).collect(),
		Err(e) => {
			eprintln!("Failed to find packages: {e:?}");
			process::exit(1);
		}
	};

	let installs = DeviceInstallations::build_requests(&devices, &packages);
	match installs.len() {
		0 => {
			eprintln!("No installation requests found.");
			process::exit(1);
		}
		device_count => {
			let total_installs = installs.iter().fold(0, |acc, e| acc + e.count());
			println!("Running {total_installs} installations on {device_count} devices...");
			let streams = installs.into_iter().map(DeviceInstallations::perform);
			let mut stream = stream::select_all(streams);
			while let Some(outcome) = stream.next().await {
				match outcome {
					Ok(o) => println!("{o}"),
					Err(e) => eprintln!("{e}"),
				}
			}
		}
	}
}
