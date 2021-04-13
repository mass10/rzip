//!
//! 様々な汎用操作
//!

use chrono::{Datelike, Timelike};

/// ディレクトリまたはファイルを削除します。
pub fn unlink(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	if path == "" {
		return Ok(());
	}
	if path == "/" {
		return Ok(());
	}
	if path == "C:" {
		return Ok(());
	}
	if path == "C:\\" {
		return Ok(());
	}
	if path == "C:\\Windows" {
		return Ok(());
	}

	let e = std::path::Path::new(path);
	if !e.exists() {
		return Ok(());
	}

	if e.is_dir() {
		std::fs::remove_dir_all(path)?;
		return Ok(());
	} else if e.is_file() {
		std::fs::remove_file(path)?;
		return Ok(());
	}
	return Ok(());
}

pub fn convert_datetime1(time: std::time::SystemTime) -> chrono::DateTime<chrono::Local> {
	return chrono::DateTime::<chrono::Local>::from(time);
}

pub fn convert_datetime2(time: chrono::DateTime<chrono::Local>) -> zip::DateTime {
	let year = time.year() as u16;
	let month = time.month() as u8;
	let day = time.day() as u8;
	let hour = time.hour() as u8;
	let min = time.minute() as u8;
	let sec = time.second() as u8;

	return zip::DateTime::from_date_and_time(year, month, day, hour, min, sec).unwrap();
}

pub fn convert_datetime0(time: std::time::SystemTime) -> zip::DateTime {
	let val1 = convert_datetime1(time);
	let val2 = convert_datetime2(val1);
	return val2;
}

/// フルパスに変換
pub fn canonicalize_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
	use crate::helpers::PathHelper;

	let path = std::path::Path::new(path);
	return path.canonical_path_as_string();
}

pub fn build_path(base_name: &str, name: &str) -> String {
	let unknown = std::path::Path::new(base_name);
	let path_name = unknown.join(name);
	return path_name.to_str().unwrap().to_string();
}

/// タイムスタンプ "%Y-%m-%d %H:%M:%S%.3f" を返します。
#[allow(unused)]
pub fn timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// タイムスタンプ "%Y%m%d-%H%M%S" を返します。
pub fn timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}
