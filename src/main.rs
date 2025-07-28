fn main() {
	if !adb_installer::find_adb() {
		eprintln!("ADB not found. Please ensure that ADB is installed.");
		return;
	}

	let version = loop {
		println!("Which version would you like to install?");
		match adb_installer::get_version() {
			Some(version) => break version,
			None => eprint!("Wrong version format. Example: 5.1. "),
		}
	};

	println!("Version: {version:?}");
}
