use crate::error::Error;
use crate::package::PackageConfig;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Write;

const CONFIG_PATH: &str = "Batch APK Installer";
const CONFIG_FILE: &str = "config.toml";
const CONFIG_TEMPLATE: &str = r#"directory = "/Users/user_name/Desktop"
platforms = [ "quest", "pico" ]

[[packages]]
id = "com.company.product.app"
platforms = [ "pico", "quest" ]
match_file_name = false

[[packages]]
id = "com.company.product.pico_only_app"
platforms = [ "pico" ]
match_file_name = true

[[packages]]
id = "com.company.product.quest_only_app"
platforms = [ "pico" ]
match_file_name = true"#;

pub type Platform = String;
pub type PackageID = String;

#[derive(Deserialize)]
pub struct Config {
	directory: String,
	platforms: Vec<Platform>,
	packages: Vec<PackageConfig>,
}

impl Config {
	pub fn build() -> Result<Config, Error> {
		let config_folder_path = dirs::config_dir().ok_or(Error::NoHomeDirectory)?;
		let app_folder_path = config_folder_path.join(CONFIG_PATH);
		let file_path = app_folder_path.join(CONFIG_FILE);
		let Ok(config) = fs::read_to_string(&file_path) else {
			if !fs::exists(&app_folder_path)? {
				fs::create_dir_all(&app_folder_path)?;
			}
			let mut file = File::create(&file_path)?;
			file.write_all(CONFIG_TEMPLATE.as_bytes())?;
			let file_path = file_path.to_str().ok_or(Error::InvalidConfigPath)?;
			println!("Config file not found. Created one at {file_path}. Modify it and try again");
			return Err(Error::ConfigNotFound);
		};

		let config = toml::from_str(config.as_str())?;
		Ok(config)
	}

	pub fn directory(&self) -> &str {
		&self.directory
	}

	pub fn platforms(&self) -> &[Platform] {
		&self.platforms
	}

	pub fn packages(&self) -> &[PackageConfig] {
		&self.packages
	}
}
