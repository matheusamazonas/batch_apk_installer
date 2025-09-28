use crate::config::{PackageID, Platform};
use crate::error::Error;
use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct Package {
	id: PackageID,
	path: String,
	platforms: Vec<Platform>,
}

#[derive(Debug)]
pub struct PackageFile {
	path: String,
	id: PackageID,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackageConfig {
	id: PackageID,
	platforms: Vec<Platform>,
}

impl Package {
	pub fn id(&self) -> &PackageID {
		&self.id
	}

	pub fn path(&self) -> &str {
		&self.path
	}

	pub fn platforms(&self) -> &[Platform] {
		&self.platforms
	}
}

impl PackageFile {
	pub fn find_all(dir: &str, configs: &[PackageConfig]) -> Result<Vec<Package>, Error> {
		fn build_package(file: PackageFile, configs: &[PackageConfig]) -> Option<Package> {
			let config = configs.iter().find(|c| c.id == file.id)?;
			let package = Package {
				id: file.id,
				path: file.path,
				platforms: config.platforms.clone(),
			};
			Some(package)
		}

		let files = find_apk_files(dir)?
			.into_iter()
			.filter_map(|f| get_package_file(&f).ok())
			.filter_map(|f| build_package(f, configs))
			.collect();
		Ok(files)
	}
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
		Err(Error::Package(String::from(
			"Failed to get APK package name.",
		)))
	}
}
