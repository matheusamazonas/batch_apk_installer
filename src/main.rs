use std::env;
use std::process;

fn main() {
	if !adb_installer::find_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		return;
	}

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Missing arguments: version.");
		process::exit(1);
	}

	let version_arg = args[1].clone();
	let version = match adb_installer::parse_version(&version_arg) {
		Some(version) => version,
		None => {
			eprintln!("Wrong version format. Example: 5.1");
			process::exit(1)
		}
	};

	println!("Version: {version:?}");
}
