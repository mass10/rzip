//!
//! Utilities.
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
pub fn build_archive_internal_path(parent: &str, name: &str) -> String {
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
#[allow(unused)]
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

///
/// Stopwatch
///
pub struct Stopwatch {
	/// remains current timestamp.
	time: std::time::Instant,
}

impl Stopwatch {
	/// Returns a new instance of `Stopwatch`.
	///
	/// ### Returns
	/// A new instance of `Stopwatch`.
	pub fn new() -> Stopwatch {
		return Stopwatch { time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// Implements default behavior as [std::fmt::Display].
	///
	/// ### Returns
	/// Duration as formatted string.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let elapsed = std::time::Instant::now() - self.time;
		write!(f, "{}", elapsed.to_string())?;
		return Ok(());
	}
}

///
/// [std::fs::DirEntry] helper methods
///
pub trait DirEntityHelper {
	/// Get the path as [String]
	///
	/// # Returns
	/// path as [String]
	fn path_as_string(&self) -> String;
}

impl DirEntityHelper for std::fs::DirEntry {
	/// Get the path as [String]
	///
	/// # Returns
	/// path as [String]
	fn path_as_string(&self) -> String {
		let tmp = self.path();
		return tmp.to_str().unwrap().to_string();
	}
}

///
/// [std::path::Path] helper methods
///
pub trait PathHelper {
	/// Get the name as &str
	///
	/// # Returns
	/// name as &str
	fn name_as_str(&self) -> &str;

	/// Get the name as [String]
	///
	/// # Returns
	/// name as [String]
	fn name_as_string(&self) -> String;

	/// Get canonical path as [String]
	///
	/// # Returns
	/// canonical path as [String]
	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>>;

	fn join_as_string(&self, child: &str) -> Result<String, Box<dyn std::error::Error>>;
}

fn fix_unc_path(path: &str) -> String {
	if !path.starts_with("\\\\?\\") {
		return path.to_string();
	}
	let mut tmp = path.to_string();
	tmp = tmp.replace("\\\\?\\", "");
	return tmp;
}

/// Concat path string.
pub fn concat_path(parent: &str, child: &str) -> String {
	if parent == "" {
		return child.to_string();
	}
	let path = std::path::Path::new(parent);
	return path.join(child).to_str().unwrap().to_string();
}

impl PathHelper for std::path::Path {
	/// Get the name as &str
	///
	/// # Returns
	/// name as &str
	fn name_as_str(&self) -> &str {
		return self.file_name().unwrap().to_str().unwrap();
	}

	/// Get the name as [String]
	///
	/// # Returns
	/// name as [String]
	fn name_as_string(&self) -> String {
		return self.file_name().unwrap().to_str().unwrap().to_string();
	}

	/// Get canonical path as [String]
	///
	/// # Returns
	/// canonical path as [String]
	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
		let path = self.canonicalize()?;
		let s = path.to_str().unwrap().to_string();
		return Ok(fix_unc_path(&s));
	}

	/// Join path as [String]
	///
	/// # Returns
	/// joined path as [String]
	fn join_as_string(&self, child: &str) -> Result<String, Box<dyn std::error::Error>> {
		let result = self.join(child);
		let s = result.to_str().unwrap().to_string();
		return Ok(s);
	}
}

///
/// [std::time::Duration] helper methods
///
pub trait DurationFormatter {
	/// Format duration as [String]
	///
	/// # Returns
	/// formatted duration as [String]
	fn to_string(&self) -> String;
}

impl DurationFormatter for std::time::Duration {
	/// Format duration as [String]
	///
	/// # Returns
	/// formatted duration as [String]
	fn to_string(&self) -> String {
		let mut millis = self.as_millis();
		let mut sec = 0;
		let mut min = 0;
		let mut hour = 0;

		while 1000 <= millis {
			sec += 1;
			millis -= 1000;
		}
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}

		let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
		return s;
	}
}

///
/// [chrono::DateTime] helper methods
///
trait ChronoDateTimeHelper {
	fn as_ziptime(&self) -> zip::DateTime;
}

impl ChronoDateTimeHelper for chrono::DateTime<chrono::Local> {
	fn as_ziptime(&self) -> zip::DateTime {
		use chrono::{Datelike, Timelike};

		let time = *self;
		let year = time.year() as u16;
		let month = time.month() as u8;
		let day = time.day() as u8;
		let hour = time.hour() as u8;
		let min = time.minute() as u8;
		let sec = time.second() as u8;

		return zip::DateTime::from_date_and_time(year, month, day, hour, min, sec).unwrap();
	}
}

///
/// [std::time::SystemTime] helper methods
///
pub trait SystemTimeHelper {
	fn as_ziptime(&self) -> zip::DateTime;
}

impl SystemTimeHelper for std::time::SystemTime {
	fn as_ziptime(&self) -> zip::DateTime {
		let val1 = chrono::DateTime::<chrono::Local>::from(*self);
		let val2 = val1.as_ziptime();
		// let val2 = convert_datetime2(val1);
		return val2;
	}
}
