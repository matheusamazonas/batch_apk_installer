fn main() {
	if !adb_installer::find_adb() {
		println!("ADB not found. Please ensure that ADB is installed.");
		return;
	}

	let version = loop {
		println!("Which version would you like to install?");
		match adb_installer::get_version() {
			Some(version) => break version,
			None => print!("Wrong version format. Example: 5.1. "),
		}
	};

	println!("Version: {version:?}");
}
