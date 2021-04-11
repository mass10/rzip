use super::application_error::ApplicationError;
use chrono::{Datelike, Timelike};
use std::io::Read;

#[allow(unused)]
fn get_file_name(path: &str) -> String {
	let unknown = std::path::Path::new(path);
	return unknown.file_name().unwrap().to_str().unwrap().to_string();
}

/// フルパスに変換
fn canonicalize_path(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let path = std::path::Path::new(path);
	let path = path.canonicalize()?;
	let path = path.to_str().unwrap().to_string();
	return Ok(path);
}

/// ディレクトリまたはファイルを削除します。
fn unlink(path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
	}
	if e.is_file() {
		std::fs::remove_file(path)?;
		return Ok(());
	}
	return Ok(());
}

fn convert_datetime1(time: std::time::SystemTime) -> chrono::DateTime<chrono::Local> {
	return chrono::DateTime::<chrono::Local>::from(time);
}

fn convert_datetime2(time: chrono::DateTime<chrono::Local>) -> zip::DateTime {
	return zip::DateTime::from_date_and_time(time.year() as u16, time.month() as u8, time.day() as u8, time.hour() as u8, time.minute() as u8, time.second() as u8).unwrap();
}

fn convert_datetime0(time: std::time::SystemTime) -> zip::DateTime {
	let val1 = convert_datetime1(time);
	let val2 = convert_datetime2(val1);
	return val2;
}

fn build_path(base_name: &str, name: &str) -> String {
	let unknown = std::path::Path::new(base_name);
	let path_name = unknown.join(name);
	return path_name.to_str().unwrap().to_string();
}

/// 正規表現による文字列のマッチング
fn matches(pattern: &str, text: &str) -> bool {
	let reg = regex::Regex::new(pattern);
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

/// ファイル名の検証
pub fn is_valid_file(dir: &std::path::Path) -> bool {
	let name = dir.file_name().unwrap().to_str().unwrap();
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches(".VC.db$", name) {
		return false;
	}
	// で終わる
	if matches(".ipch$", name) {
		return false;
	}
	return true;
}

/// ディレクトリの妥当性を検証します。
pub fn is_valid_directory(dir: &std::path::Path) -> bool {
	match dir.file_name().unwrap().to_str().unwrap() {
		"node_modules" => return false,
		".git" => return false,
		"dist" => return false,
		".nuxt" => return false,
		"Debug" => return false,
		"Release" => return false,
		"ReleaseDebug" => return false,
		"target" => return false,
		"ipch" => return false,
		"x64" => return false,
		_ => {}
	};
	return true;
}

pub struct Application;

impl Application {
	pub fn new() -> Application {
		let instance = Application {};
		return instance;
	}

	fn append_entry(&self, archiver: &mut zip::ZipWriter<std::fs::File>, base_name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		use std::io::Write;

		let unknown = std::path::Path::new(path);
		if unknown.is_dir() {
			if !is_valid_directory(unknown) {
				return Ok(());
			}

			let name = unknown.file_name().unwrap().to_str().unwrap();

			// 内部ディレクトリエントリーを作成
			if base_name != "" {
				println!("adding file ... {}", &base_name);
				let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
				archiver.add_directory(build_path(base_name, name), options)?;
			}

			let it = std::fs::read_dir(path)?;
			for e in it {
				let entry = e?;

				// base_name からの内部相対パスを生成
				let sub_dir_name = build_path(base_name, &name);

				let path_name = entry.path();
				let path_name = path_name.to_str().unwrap();

				self.append_entry(archiver, &sub_dir_name, path_name)?;
			}
		} else if unknown.is_file() {
			if !is_valid_file(unknown) {
				return Ok(());
			}
			let full_path = unknown.to_str().unwrap();
			let name = unknown.file_name().unwrap().to_str().unwrap();

			let relative_path = build_path(base_name, name);

			println!("adding file ... {}", &relative_path);

			let meta = unknown.metadata()?;

			// ファイルをアーカイブ
			let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

			// 最終更新日時
			let last_modified = meta.modified()?;
			let last_modified = convert_datetime0(last_modified);
			let options = options.last_modified_time(last_modified);

			archiver.start_file(&relative_path, options)?;

			let mut stream = std::fs::File::open(full_path)?;
			loop {
				let mut buffer = [0; 1000];
				let bytes_read = stream.read(&mut buffer)?;
				if bytes_read == 0 {
					break;
				}
				let write_buffer = &buffer[..bytes_read];
				archiver.write(&write_buffer)?;
			}
		} else {
			let message = format!("Unknown filesystem [{}].", path);
			let err = ApplicationError::new(&message);
			return Err(Box::new(err));
		}

		return Ok(());
	}

	/// ディレクトリをアーカイブします。
	///
	/// # Arguments
	/// `path` パス
	pub fn archive(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// パスを正規化
		let path = canonicalize_path(path)?;

		println!("archiving ... {}", &path);

		// ファイル名を生成
		let archive_path_name = path.to_string() + ".zip";

		// 起点となるディレクトリの名前
		let base_name = "";

		// .zip ファイルがあれば削除
		unlink(&archive_path_name)?;

		// アーカイバーの初期化
		let w = std::fs::File::create(archive_path_name)?;
		let mut archiver = zip::ZipWriter::new(w);

		// ここから走査
		self.append_entry(&mut archiver, &base_name, &path)?;

		// アーカイバーを閉じます。
		archiver.finish()?;

		return Ok(());
	}
}
