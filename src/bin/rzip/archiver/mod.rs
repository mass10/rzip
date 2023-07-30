//!
//! Zip archiver module
//!

use crate::configuration;
use crate::util;

/// Zip archiver class
pub struct ZipArchiver {
	archiver: zip::ZipWriter<std::fs::File>,
}

impl ZipArchiver {
	/// Returns a new instance of [ZipArchiver].
	pub fn new(path_to_archive: &str) -> Result<ZipArchiver, Box<dyn std::error::Error>> {
		let file = std::fs::File::create(path_to_archive)?;
		let archiver = zip::ZipWriter::new(file);
		let instance = ZipArchiver { archiver };
		return Ok(instance);
	}

	/// Create a new entry into archive.
	///
	/// # Arguments
	/// * `base_name` Relative path of folder.
	/// * `path` Path to a new entry.
	/// * `create_root` Whether it creates root folder or not.
	pub fn append(&mut self, base_name: &str, path: &str, settings: &configuration::Settings, create_root: bool) -> Result<(), Box<dyn std::error::Error>> {
		use crate::util::DirEntityHelper;
		use crate::util::PathHelper;
		use std::io::Read;
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
			let internal_path = if create_root { util::build_archive_internal_path(base_name, name) } else { String::new() };

			// Create directory node.
			if create_root {
				let meta = unknown.metadata()?;

				// Create directory attributes.
				let options = create_file_attributes(&meta)?;

				// Relative path from the root. "path/to/name"
				let internal_path = format!("{}/", internal_path);

				println!("  adding: {} (stored)", &internal_path);

				self.archiver.add_directory(&internal_path, options)?;
			}

			// enumerate sub entries.
			let it = std::fs::read_dir(path)?;
			for e in it {
				let entry = e?;
				let fullpath = entry.path_as_string();
				self.append(&internal_path, &fullpath, &settings, true)?;
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

			// Create file attributes.
			let options = create_directory_attributes(&meta)?;

			// Relative path from the root. "path/to/name"
			let internal_path = util::build_archive_internal_path(base_name, name);

			println!("  adding: {} (deflated)", &internal_path);

			self.archiver.start_file(&internal_path, options)?;
			let mut stream = std::fs::File::open(path)?;
			loop {
				let mut buffer = [0; 4000];
				let bytes_read = stream.read(&mut buffer)?;
				if bytes_read == 0 {
					break;
				}
				let write_buffer = &buffer[..bytes_read];
				self.archiver.write_all(&write_buffer)?;
			}
		} else {
			let message = format!("Unknown filesystem [{}].", path);
			return Err(message.into());
		}

		return Ok(());
	}
}

impl Drop for ZipArchiver {
	fn drop(&mut self) {
		let _ = self.archiver.finish();
	}
}

/// Retrieve unix permissions as u8
#[allow(unused)]
fn get_unix_permissions_as_u8(meta: &std::fs::Metadata) -> Option<u32> {
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let perm = meta.permissions();
		let mode = perm.mode();
		return Some(mode);
	}

	return None;
}

/// Create file attributes.
fn create_file_attributes(meta: &std::fs::Metadata) -> Result<zip::write::FileOptions, Box<dyn std::error::Error>> {
	use crate::util::SystemTimeHelper;

	let options = zip::write::FileOptions::default();

	let options = options.compression_method(zip::CompressionMethod::Stored);

	// last modified time
	let last_modified = meta.modified()?.as_ziptime();
	let options = options.last_modified_time(last_modified);

	return Ok(options);
}

/// Create directory attributes.
fn create_directory_attributes(meta: &std::fs::Metadata) -> Result<zip::write::FileOptions, Box<dyn std::error::Error>> {
	use crate::util::SystemTimeHelper;

	let options = zip::write::FileOptions::default();

	// compression method
	let options = options.compression_method(zip::CompressionMethod::Deflated);

	// last modified time
	let last_modified = meta.modified()?.as_ziptime();
	let options = options.last_modified_time(last_modified);

	// permissions
	let options = match get_unix_permissions_as_u8(&meta) {
		None => options,
		Some(n) => options.unix_permissions(n),
	};

	return Ok(options);
}
