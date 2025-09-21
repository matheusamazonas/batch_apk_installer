use crate::config::Config;
use crate::error::Error;
use std::env;
use std::process;

mod android;
mod config;
mod error;

fn main() {
	if !android::has_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	if !android::has_aapt() {
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
		Err(Error::ConfigError(e)) => {
			eprintln!("Config error: {e}.");
			process::exit(1);
		}
		Err(e) => {
			eprintln!("Error when loading config: {e:?}");
			process::exit(1)
		}
	};

	let devices = match android::get_devices() {
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

	for device in devices {
		println!("Found device: {device:?}");
	}

	let apks = match android::find_package_files(&config.directory) {
		Ok(apks) => apks,
		Err(e) => {
			eprintln!("Failed to find packages: {e:?}");
			process::exit(1);
		}
	};
	for apk in apks {
		println!("Found package: {apk:?}")
	}
}
