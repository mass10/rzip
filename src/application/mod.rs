//!
//! アプリケーション本体を含む、アプリケーションコア部分の実装です。
//!

use std::io::Read;

use crate::{configuration, functions, helpers::SystemTimeHelper};

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

			// Relative path from the root. "path/to/name"
			let internal_path = functions::build_archive_internal_path(base_name, name);

			// Create directory node.
			{
				println!("  adding: {} (stored)", &internal_path);

				// last modified time
				let last_modified = unknown.metadata()?.modified()?.as_ziptime();

				let options = zip::write::FileOptions::default();
				let options = options.compression_method(zip::CompressionMethod::Stored);
				let options = options.last_modified_time(last_modified);

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
			// name of file
			let name = unknown.name_as_str();
			// validate its name
			if !settings.is_valid_filename(name)? {
				println!("[INFO] IGNORE {}", name);
				return Ok(());
			}

			let meta = unknown.metadata()?;

			let options = zip::write::FileOptions::default();
			// Relative path from the root. "path/to/name"
			let internal_path = functions::build_archive_internal_path(base_name, name);
			// compression method
			let options = options.compression_method(zip::CompressionMethod::Deflated);
			// last modified time
			let last_modified = meta.modified()?.as_ziptime();
			let options = options.last_modified_time(last_modified);

			// 内部構造にファイルエントリーを作成
			println!("  adding: {} (deflated)", &internal_path);
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
			return Err(message.into());
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
