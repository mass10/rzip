//!
//! Configuration
//!

extern crate serde_derive;

use crate::util;

/// Fix "some" to "^some". "*some" is not changed.
fn head(wildcard: &str) -> String {
	if wildcard.starts_with("*") {
		return wildcard.to_string();
	}
	return format!("^{}", wildcard);
}

/// Fix "some" to "some$". "some*" is not changed.
fn tail(wildcard: &str) -> String {
	if wildcard.ends_with("*") {
		return wildcard.to_string();
	}
	return format!("{}$", wildcard);
}

/// Make name filter from wildcard.
fn make_name_filter(wildcard: &str) -> String {
	let wildcard = head(wildcard);
	let wildcard = tail(&wildcard);
	let wildcard = wildcard.replace("[", "\\[");
	let wildcard = wildcard.replace("]", "\\]");
	let wildcard = wildcard.replace(".", "\\.");
	let wildcard = wildcard.replace("*", ".+");
	return wildcard;
}

fn get_env(name: &str) -> String {
	let value = std::env::var(name);
	if value.is_err() {
		return "".to_string();
	}
	return value.unwrap();
}

/// Detect the user's home directory.
fn detect_users_home_dir() -> String {
	// (Windows) Detect the user's home directory.
	let home = get_env("USERPROFILE");
	if home != "" {
		return home;
	}

	// (Linux) Detect the user's home directory.
	let home = get_env("HOME");
	if home != "" {
		return home;
	}

	return "".to_string();
}

/// Detect "settings.toml" in the current directory or the user's home directory.
fn find_settings_toml() -> Result<String, Box<dyn std::error::Error>> {
	const NAME: &str = "settings.toml";

	// Detect "settings.toml" in the current directory.
	if std::path::Path::new(NAME).is_file() {
		return Ok(NAME.to_string());
	}

	// Detect the user's home directory.
	let home = detect_users_home_dir();
	if home == "" {
		return Ok("".to_string());
	}

	let path = util::concat_path(&home, NAME);
	if !std::path::Path::new(&path).is_file() {
		return Ok("".to_string());
	}

	return Ok(path);
}

///
/// Structure for Settings
///
#[derive(serde_derive::Deserialize, std::fmt::Debug, std::clone::Clone)]
pub struct Settings {
	/// Dirs to exclude.
	pub exclude_dirs: Option<std::collections::HashSet<String>>,

	/// Files to exclude.
	pub exclude_files: Option<std::collections::HashSet<String>>,
}

impl Settings {
	/// Create a new instance.
	pub fn new() -> Result<Settings, Box<dyn std::error::Error>> {
		let mut instance = Settings {
			exclude_dirs: Some(std::collections::HashSet::new()),
			exclude_files: Some(std::collections::HashSet::new()),
		};

		// Detect "settings.toml" in the current directory or the user's home directory.
		let path = find_settings_toml()?;

		// Configure
		instance.configure(&path)?;

		return Ok(instance);
	}

	/// Configure
	///
	/// # Arguments
	/// * `path` Path to "settings.toml"
	fn configure(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Skip if path is empty.
		if path == "" {
			return Ok(());
		}

		// Skip if the file does not exist.
		if !std::path::Path::new(path).is_file() {
			println!("[INFO] Configuration file not found. (settings.toml)");
			return Ok(());
		}

		// テキストファイル全体を読み込み
		let content = util::read_text_file_all(&path)?;

		// toml ファイルをパース
		*self = toml::from_str(&content)?;
		if self.exclude_dirs.is_none() {
			self.exclude_dirs = Some(std::collections::HashSet::new());
		}
		if self.exclude_files.is_none() {
			self.exclude_files = Some(std::collections::HashSet::new());
		}

		return Ok(());
	}

	/// 指定された名前が処理対象か調べます。
	///
	/// # Arguments
	/// * `name` ディレクトリの名前
	///
	/// # Returns
	/// 処理対象(=つまり除外ディレクトリ名に指定されていない)なら `true` を返します。
	pub fn is_valid_dir(&self, name: &str) -> bool {
		if self.exclude_dirs.is_none() {
			return true;
		}
		let names = self.exclude_dirs.as_ref().unwrap();
		for e in names {
			if name == e {
				return false;
			}
		}
		return true;
	}

	/// 指定された名前が処理対象のファイルか調べます。
	///
	/// # Arguments
	/// * `name` ファイルの名前
	///
	/// # Returns
	/// 処理対象(=つまり除外ファイル名に指定されていない)なら `true` を返します。
	pub fn is_valid_filename(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
		if self.exclude_files.is_none() {
			return Ok(true);
		}

		let names = self.exclude_files.as_ref().unwrap();
		for e in names {
			let wildcard = make_name_filter(&e);
			let regex = regex::Regex::new(&wildcard)?;
			let matched = regex.is_match(name);
			if matched {
				return Ok(false);
			}
			if name == e {
				return Ok(false);
			}
		}

		return Ok(true);
	}
}
