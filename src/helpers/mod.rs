//!
//! Utility traits
//!

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
		let cano = self.canonicalize()?;
		let s = cano.to_str().unwrap().to_string();
		return Ok(s);
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
