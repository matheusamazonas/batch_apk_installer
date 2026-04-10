use crate::config::Config;
use crate::error::Error;
use crate::installation::DeviceInstallations;
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
	eprintln!("\x1b[91m{error}\x1b[0m");
}

fn get_parameters(args: &[String]) -> Result<(String, bool), Error> {
	if !has_adb() {
		return Err(Error::MissingADB);
	}

	if !has_aapt() {
		return Err(Error::MissingAAPT);
	}

	let Some(packages_folder) = args.get(1) else {
		return Err(Error::MissingVersionArgument);
	};

	let uninstall = match args.get(2) {
		Some(arg) => arg == "-u",
		None => false,
	};

	Ok((packages_folder.clone(), uninstall))
}

#[tokio::main]
async fn main() {
	let args: Vec<String> = env::args().collect();
	if args.contains(&String::from("-h")) {
		println!(
			"Usage: <batch_apk_installer> <version> [options...]\n\
			Where:\n\
			\t<batch_apk_installer> is the name of the binary.\n\
			\t<version> is the version in the semantic versioning format (e.g. 2.1 and 4.1.2). \n\
			And the following options are available:\n\
			\t-u\twhether the packages should be uninstalled from the devices before being \
			installed. \n\
		    \t-h\tdisplays the help text (this one)."
		);
		process::exit(0);
	}

	let (packages_folder, uninstall) = match get_parameters(&args) {
		Ok((packages_folder, uninstall)) => (packages_folder, uninstall),
		Err(e) => {
			print_error(&e.to_string());
			process::exit(1);
		}
	};

	let config = match Config::build() {
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

	let packages_dir = PathBuf::from(config.directory()).join(packages_folder);
	let packages: Vec<_> = match package::find_all_packages(&packages_dir, config.packages()) {
		Ok(packages) => packages.into_iter().map(Arc::new).collect(),
		Err(e) => {
			print_error(&e.to_string());
			process::exit(1);
		}
	};

	if packages.is_empty() {
		print_error("No packages found.");
		process::exit(1);
	}

	let installs = DeviceInstallations::build_requests(&devices, &packages, uninstall);
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
					None => println!("\x1b[92m{description} completed successfully.\x1b[0m"),
				}
			}
		}
	}
}
