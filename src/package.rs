use crate::config::{PackageID, Platform};
use crate::error::Error;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct Package {
	id: PackageID,
	path: String,
	file_name: String,
	platforms: Vec<Platform>,
	match_file_name: bool,
}

pub struct PackageFile {
	path: PathBuf,
	id: PackageID,
}

#[derive(Deserialize)]
pub struct PackageConfig {
	id: PackageID,
	platforms: Vec<Platform>,
	match_file_name: bool,
}

impl Package {
	pub fn try_new(
		file: PackageFile, platforms: Vec<Platform>, match_file_name: bool,
	) -> Option<Package> {
		let file_name = file.path.file_name()?.to_str()?.to_string();
		let path = file.path.into_os_string().into_string().ok()?;
		let package = Package {
			id: file.id,
			path,
			file_name,
			platforms,
			match_file_name,
		};
		Some(package)
	}

	pub fn id(&self) -> &PackageID {
		&self.id
	}

	pub fn path(&self) -> &String {
		&self.path
	}

	pub fn platforms(&self) -> &[Platform] {
		&self.platforms
	}

	pub fn file_name(&self) -> &String {
		&self.file_name
	}

	pub fn match_file_name(&self) -> bool {
		self.match_file_name
	}
}

pub fn find_all_packages(dir: &PathBuf, configs: &[PackageConfig]) -> Result<Vec<Package>, Error> {
	fn build_package(file: PackageFile, configs: &[PackageConfig]) -> Option<Package> {
		let config = configs.iter().find(|c| c.id == file.id)?;
		let package = Package::try_new(file, config.platforms.clone(), config.match_file_name)?;
		Some(package)
	}

	if let Ok(true) = fs::exists(dir) {
		let files = find_apk_files(dir)?
			.into_iter()
			.filter_map(|f| get_package_file(&f).ok())
			.filter_map(|f| build_package(f, configs))
			.collect();
		Ok(files)
	} else {
		let path = dir.to_string_lossy().to_string();
		Err(Error::NoPackageDirectory(path))
	}
}

fn find_apk_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
	let entries = fs::read_dir(dir)?
		.filter_map(Result::ok)
		.map(|e| e.path())
		.filter(|path| path.extension().is_some_and(|ext| ext == "apk"))
		.collect();
	Ok(entries)
}

fn get_package_file(path: &PathBuf) -> Result<PackageFile, Error> {
	let path_str = path.to_str().ok_or(Error::MalformedPackageFilePath)?;
	let output = Command::new("aapt2")
		.args(["dump", "packagename", path_str])
		.output()?;
	if output.status.success() {
		let output = String::from_utf8(output.stdout)?;
		let id = String::from(output.trim_end());
		let path = PathBuf::from(path);
		let package = PackageFile { path, id };
		Ok(package)
	} else {
		Err(Error::PackageNameNotFound)
	}
}
