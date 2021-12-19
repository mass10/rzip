//!
//! Various functions.
//!

/// Remove directory or file specified by `path`.
///
/// # Arguments
/// * `path` Path to remove.
pub fn unlink(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	if path == "" {
		return Ok(());
	}
	let e = std::path::Path::new(path);
	if e.is_dir() {
		// Remove directory.
		std::fs::remove_dir_all(path)?;
	} else if e.is_file() {
		// Remove file.
		std::fs::remove_file(path)?;
	}
	return Ok(());
}

/// Get canonical path of `path`.
pub fn canonicalize_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
	use crate::helpers::PathHelper;

	let path = std::path::Path::new(path);
	return path.canonical_path_as_string();
}

/// Build a path from `path` and `name`.
///
/// # Arguments
/// * `parent` Parent path.
/// * `name` Name of the file or directory.
///
/// # Returns
/// * Path to the file or directory.
pub fn build_path(parent: &str, name: &str) -> String {
	if parent == "" {
		return name.to_string();
	}
	return format!("{}/{}", parent, name);
}

/// Get current time as a string in the format `%Y-%m-%d %H:%M:%S%.3f`.
///
/// # Returns
/// Current time as a [String]
#[allow(unused)]
pub fn timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// Get current time as a string in the format `%Y%m%d-%H%M%S`.
///
/// # Returns
/// Current time as a [String]
pub fn timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

/// Retrieve the whole content of file
///
/// ### Returns
/// Entire content of file as `String`
pub fn read_text_file_all(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use std::io::Read;

	let mut file = std::fs::File::open(path)?;
	let mut s = String::new();
	file.read_to_string(&mut s)?;
	return Ok(s);
}
