use crate::error::Error;
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct PackageFile {
	path: String,
	id: String,
}

pub fn find_package_files(dir: &str) -> Result<Vec<PackageFile>, Error> {
	let files = find_apk_files(dir)?
		.into_iter()
		.filter_map(|f| get_package_file(&f).ok())
		.collect();
	Ok(files)
}

fn find_apk_files(dir: &str) -> Result<Vec<String>, Error> {
	let entries = fs::read_dir(dir)?
		.filter_map(|e| e.ok())
		.map(|e| e.path())
		.filter(|path| path.extension().is_some_and(|ext| ext == "apk"))
		.filter_map(|path| path.into_os_string().into_string().ok())
		.collect();
	Ok(entries)
}

fn get_package_file(path: &str) -> Result<PackageFile, Error> {
	let output = Command::new("aapt2")
		.args(["dump", "packagename", path])
		.output()?;
	if output.status.success() {
		let output = String::from_utf8(output.stdout)?;
		let id = String::from(output.trim_end());
		let path = String::from(path);
		let package = PackageFile { path, id };
		Ok(package)
	} else {
		Err(Error::AaptError(String::from(
			"Failed to get APK package name.",
		)))
	}
}
