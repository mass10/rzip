//!
//! Application core implementation
//!

use super::errors::ApplicationError;
use crate::configuration;
use crate::functions;
use std::io::Read;

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

	/// Create a new entry into archive.
	///
	/// # Arguments
	/// * `archiver` [zip::ZipWriter].
	/// * `base_name` Relative path of folder.
	/// * `path` Path to a new entry.
	fn append_entry(&self, archiver: &mut zip::ZipWriter<std::fs::File>, base_name: &str, path: &str, settings: &configuration::Settings) -> Result<(), Box<dyn std::error::Error>> {
		use crate::helpers::DirEntityHelper;
		use crate::helpers::PathHelper;
		use std::io::Write;

		let unknown = std::path::Path::new(path);
		if unknown.is_dir() {
			// name of directory
			let name = unknown.name_as_str();
			// validate its name
			if !settings.is_valid_dir(name) {
				println!("[INFO] IGNORE {}", name);
				return Ok(());
			}

			// relative path from the root.
			let internal_path = functions::build_path(base_name, name);

			// crate directory tree if needed.
			if base_name != "" {
				println!("[INFO] adding ... {}", &base_name);
				let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
				archiver.add_directory(&internal_path, options)?;
			}

			// enumerate sub entries.
			let it = std::fs::read_dir(path)?;
			for e in it {
				let entry = e?;
				let fullpath = entry.path_as_string();
				self.append_entry(archiver, &internal_path, &fullpath, &settings)?;
			}
		} else if unknown.is_file() {
			use crate::helpers::SystemTimeHelper;
			// name of file
			let name = unknown.name_as_str();
			// validate its name
			if !settings.is_valid_filename(name)? {
				println!("[INFO] IGNORE {}", name);
				return Ok(());
			}

			let options = zip::write::FileOptions::default();
			// ZIP ルートからの相対パス
			let internal_path = functions::build_path(base_name, name);
			// ファイルのメタ情報
			let meta = unknown.metadata()?;
			// 圧縮方法
			let options = options.compression_method(zip::CompressionMethod::Stored);
			// 最終更新日時
			let last_modified = meta.modified()?;
			let last_modified = last_modified.as_ziptime();
			let options = options.last_modified_time(last_modified);

			// 内部構造にファイルエントリーを作成
			println!("[INFO] adding ... {}", &internal_path);
			archiver.start_file(&internal_path, options)?;
			let mut stream = std::fs::File::open(path)?;
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

	/// Create a new archive.
	///
	/// # Arguments
	/// `path` Path to a directory.
	pub fn archive(&self, settings: &configuration::Settings, path_to_archive: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Canonicalize path.
		let path = functions::canonicalize_path(path)?;

		println!("[INFO] archiving ... {} >> {}", path, path_to_archive);

		// Remove existing .zip file.
		functions::unlink(path_to_archive)?;

		// Create a new archive.
		let w = std::fs::File::create(path_to_archive)?;
		let mut archiver = zip::ZipWriter::new(w);

		// Add a new entry to the archive.
		self.append_entry(&mut archiver, "", &path, &settings)?;

		// Finish archiving
		archiver.finish()?;

		return Ok(());
	}
}
