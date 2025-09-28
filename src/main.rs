use crate::config::Config;
use crate::error::Error;
use crate::installation::InstallationRequest;
use crate::package::PackageFile;
use std::env;
use std::process;
use std::process::{Command, Stdio};

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

fn main() {
	if !has_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	if !has_aapt() {
		eprintln!("AAPT not found. Please ensure that AAPT is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	}

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

	let devices = match device::get_devices(config.platforms()) {
		Ok(devices) if !devices.is_empty() => devices,
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
		println!("Found device: {device}");
	}

	let packages = match PackageFile::find_all(config.directory(), config.packages()) {
		Ok(apks) => apks,
		Err(e) => {
			eprintln!("Failed to find packages: {e:?}");
			process::exit(1);
		}
	};

	for package in &packages {
		println!("Found package file: {package:?}")
	}

	let requests = InstallationRequest::build_requests(&devices, &packages);
	for request in requests {
		let info = request.to_string();
		match request.perform() {
			Ok(_) => {
				println!("{info} succeeded.");
			}
			Err(e) => {
				println!("{info} failed with error: {e}");
			}
		}
	}
}
