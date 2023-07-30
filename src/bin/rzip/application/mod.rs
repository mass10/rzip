//!
//! アプリケーション本体を含む、アプリケーションコア部分の実装です。
//!

use std::io::Read;

use crate::archiver;
use crate::configuration;
use crate::util;

/// regex string matching
///
/// # Arguments
/// `pattern` regex pattern
/// `text` sample
/// # Returns
/// Whether it matched or else.
#[allow(unused)]
fn matches(pattern: &str, text: &str) -> bool {
	let reg = regex::Regex::new(pattern);
	if reg.is_err() {
		panic!("[ERROR] Fatal error. (reason: {})", reg.err().unwrap());
		return false;
	}
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

/// 0 padding
fn zero_pad_2(n: u32) -> String {
	return format!("{:0>2}", n);
}

/// Extract reserved keywords.
fn extract_keywords(str: &str, name: &str) -> String {
	use chrono::Datelike;
	use chrono::Timelike;

	let now = std::time::SystemTime::now();
	let now = chrono::DateTime::<chrono::Local>::from(now);

	let mut result = str.to_string();

	// for Linux
	if result.contains("%Y") {
		result = result.replace("%Y", &now.year().to_string());
	}
	if result.contains("%m") {
		result = result.replace("%m", &zero_pad_2(now.month()));
	}
	if result.contains("%d") {
		result = result.replace("%d", &zero_pad_2(now.day()));
	}
	if result.contains("%H") {
		result = result.replace("%H", &zero_pad_2(now.hour()));
	}
	if result.contains("%M") {
		result = result.replace("%M", &zero_pad_2(now.minute()));
	}
	if result.contains("%S") {
		result = result.replace("%S", &zero_pad_2(now.second()));
	}
	if result.contains("%0") {
		result = result.replace("%0", name);
	}

	// for Windows
	if result.contains("{Y}") {
		result = result.replace("{Y}", &now.year().to_string());
	}
	if result.contains("{m}") {
		result = result.replace("{m}", &zero_pad_2(now.month()));
	}
	if result.contains("{d}") {
		result = result.replace("{d}", &zero_pad_2(now.day()));
	}
	if result.contains("{H}") {
		result = result.replace("{H}", &zero_pad_2(now.hour()));
	}
	if result.contains("{M}") {
		result = result.replace("{M}", &zero_pad_2(now.minute()));
	}
	if result.contains("{S}") {
		result = result.replace("{S}", &zero_pad_2(now.second()));
	}
	if result.contains("{0}") {
		result = result.replace("{0}", name);
	}

	return result;
}

/// Read file to the end.
#[allow(unused)]
fn read_file_to_end(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let mut file = std::fs::File::open(path)?;
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;
	return Ok(buffer);
}

///
/// Application core
///
pub struct Zipper;

impl Zipper {
	/// Returns a new instance of [Zipper].
	///
	/// # Returns
	/// A new instance of [Zipper].
	pub fn new() -> Zipper {
		let instance = Zipper {};
		return instance;
	}

	/// Create a new archive.
	///
	/// # Arguments
	/// * `settings` [configuration::Settings].
	/// * `path_to_archive` Path to a new archive.
	/// * `path` Path to a directory.
	/// * `create_root` Create a root directory.
	pub fn archive(&self, settings: &configuration::Settings, path_to_archive: &str, path: &str, create_root: bool) -> Result<(), Box<dyn std::error::Error>> {
		// Canonicalize path.
		let path = util::canonicalize_path(path)?;

		let name = std::path::Path::new(&path).file_name();
		let name = name.unwrap().to_str().unwrap();

		// Extract special keywords.
		let path_to_archive = extract_keywords(path_to_archive, name);

		println!("[INFO] archiving ... {} >> {}", &path, &path_to_archive);

		// Remove existing .zip file.
		util::unlink(&path_to_archive)?;

		// Create a new archive.
		let mut archiver = archiver::ZipArchiver::new(&path_to_archive)?;
		archiver.append("", &path, &settings, create_root)?;

		return Ok(());
	}
}
