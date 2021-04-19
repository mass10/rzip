//!
//! 様々な汎用操作
//!

/// ディレクトリまたはファイルを削除します。
///
/// # Arguments
/// * `path` ファイル、またはディレクトリのパス
pub fn unlink(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	if path == "" {
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
	use chrono::{Datelike, Timelike};

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

/// 内部パスを生成します。
///
/// # Arguments
/// * `parent` エントリーを追加する場所までのパス
/// * `name` 新しいエントリーの名前
///
/// # Returns
/// パス文字列
pub fn build_path(parent: &str, name: &str) -> String {
	if parent == "" {
		return name.to_string();
	}
	return format!("{}/{}", parent, name);
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
