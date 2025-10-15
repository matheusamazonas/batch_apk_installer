use crate::config::Config;
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

fn has_adb() -> bool {
	command_exists("adb", "--version")
}

fn has_aapt() -> bool {
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

fn print_error(error: &str) {
	eprintln!("\x1b[91m{}\x1b[0m", error);
}

#[tokio::main]
async fn main() {
	if !has_adb() {
		print_error("ADB not found. Ensure that ADB is installed.");
		process::exit(1)
	}

	if !has_aapt() {
		print_error("AAPT2 not found. Ensure that AAPT2 is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	let Some(version) = args.get(1) else {
		print_error("Missing arguments: version.");
		process::exit(1);
	};

	let config = match Config::build(&args[1]) {
		Ok(config) => config,
		Err(e) => {
			let message = format!("Error when loading config: {e}.");
			print_error(&message);
			process::exit(1)
		}
	};

	let devices: Vec<_> = match device::get_devices(config.platforms()) {
		Ok(devices) if !devices.is_empty() => devices.into_iter().map(Arc::new).collect(),
		Ok(_) => {
			print_error("No devices were found.");
			process::exit(1)
		}
		Err(e) => {
			let message = format!("Error when fetching devices: {e}.");
			print_error(&message);
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
			let message = format!("Failed to find packages: {e}.");
			print_error(&message);
			process::exit(1);
		}
	};

	let installs = DeviceInstallations::build_requests(&devices, &packages);
	match installs.len() {
		0 => {
			print_error("No installation requests found.");
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
						print_error(&error);
					}
					None => println!("\x1b[92m{} completed successfully.\x1b[0m", description),
				}
			}
		}
	}
}
