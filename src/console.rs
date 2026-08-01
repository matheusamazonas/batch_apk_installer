use crate::error::Error;
use std::process::Stdio;
use std::{env, process};

const MIN_ARG_COUNT: usize = 1;
const MAX_ARG_COUNT: usize = 2;

pub enum Command {
	Help,
	Install { folder: String, uninstall: bool },
}

pub fn print_error(error: &str) {
	eprintln!("\x1b[91m{error}\x1b[0m");
}

pub fn print_warning(message: &str) {
	eprintln!("\x1b[93m{message}\x1b[0m");
}

pub fn get_command() -> Result<Command, Error> {
	if !has_adb() {
		return Err(Error::MissingADB);
	}

	if !has_aapt() {
		return Err(Error::MissingAAPT);
	}

	let args: Vec<String> = env::args().collect();
	let arg_count = args.len() - 1; // -1 because the first argument is the binary name.
	let count_within_range = (MIN_ARG_COUNT..=MAX_ARG_COUNT).contains(&arg_count);
	if !count_within_range {
		return Err(Error::WrongNumberOfArguments {
			actual: arg_count,
			min: MIN_ARG_COUNT,
			max: MAX_ARG_COUNT,
		});
	}

	if args.contains(&String::from("-h")) {
		return Ok(Command::Help);
	}

	let Some(packages_folder) = args.get(1) else {
		return Err(Error::MissingPackagesFolderArgument);
	};

	let uninstall = match args.get(2) {
		Some(arg) => match arg.as_str() {
			"-u" => true,
			other => return Err(Error::UnknownArgument(other.to_string())),
		},
		None => false,
	};

	let commands = Command::Install {
		folder: packages_folder.clone(),
		uninstall,
	};
	Ok(commands)
}

pub fn print_help() {
	println!(
		"Usage: <batch_apk_installer> <packages_folder> [options...]\n\
			Where:\n\
			\t<batch_apk_installer> is the name of the binary.\n\
			\t<packages_folder> is the name (not the path) of the folder containing the packages (APKs)\n\
			\t                  you would like to install. This folder must be a subfolder of the one\n\
			\t                  declared in the configuration's `directory` field. \n\
			And the following options are available:\n\
			\t-u\twhether the packages should be uninstalled from the devices before being \
			installed. \n\
		    \t-h\tdisplays the help text (this one)."
	);
}

fn has_adb() -> bool {
	tool_exists("adb", "--version")
}

fn has_aapt() -> bool {
	tool_exists("aapt2", "version")
}

fn tool_exists(tool_name: &str, args: &str) -> bool {
	process::Command::new(tool_name)
		.args([args])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.status()
		.is_ok()
}
