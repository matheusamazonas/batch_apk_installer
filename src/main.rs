use std::process::{Command, Stdio};

fn find_adb() -> bool {
	Command::new("adb")
		.args(["--version"])
		.stdout(Stdio::null())
		.status()
		.is_ok()
}

fn main() {
	if !find_adb() {
		println!("ADB nout found.");
		return;
	}

	println!("ADB found.");
}
