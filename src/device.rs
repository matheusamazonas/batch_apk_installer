use std::process::Command;
use crate::error::Error;

pub fn get_devices() -> Result<Vec<String>, Error> {
    let output = Command::new("adb").args(["devices", "-l"]).output()?;
    let output_str = String::from_utf8(output.stdout)?;
    let header_line_ix = output_str
        .lines()
        .position(|l| l.contains("List of devices attached"))
        .ok_or(Error::AdbError(String::from("Failed to fetch Android devices.", )))?;

    let devices: Vec<String> = output_str
        .lines()
        .skip(header_line_ix + 1)
        .filter_map(parse_device)
        .collect();
    return Ok(devices);

    fn parse_device(line: &str) -> Option<String> {
        if !line.is_empty() {
            Some(String::from(line))
        } else {
            None
        }
    }
}