use std::env;
use std::process;

fn main() {
	if !apk_installer::android::has_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		process::exit(1)
	}

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	}

	let version = match apk_installer::parse_version(&args[1]) {
		Ok(version) => version,
		Err(_) => {
			eprintln!("Wrong version format. Example: 5.1");
			process::exit(1)
		}
	};

	let devices = match apk_installer::android::get_devices() {
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
