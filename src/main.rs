use std::env;
use std::process;
use crate::config::Config;
use crate::error::Error;

mod error;
mod android;
mod config;

fn main() {
	if !android::has_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	}

	let config = match Config::build(&args[1]) {
		Ok(config) => config,
        Err(Error::CommandError(_)) => {
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
		println!("Found device: {}", device);
	}
}
