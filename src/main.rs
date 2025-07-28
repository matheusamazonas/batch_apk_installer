use std::env;
use std::process;

fn main() {
	if !adb_installer::adb::is_present() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	}

	let version = match adb_installer::parse_version(&args[1]) {
		Some(version) => version,
		None => {
			eprintln!("Wrong version format. Example: 5.1");
			process::exit(1)
		}
	};

	let devices = match adb_installer::adb::get_devices() {
		Ok(devices) if devices.len() > 0 => devices,
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
