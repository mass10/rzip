extern crate serde_derive;

use crate::functions;

fn head(wildcard: &str) -> String {
	if wildcard.starts_with("*") {
		return wildcard.to_string();
	} else {
		return format!("^{}", wildcard);
	};
}

fn tail(wildcard: &str) -> String {
	if wildcard.ends_with("*") {
		return wildcard.to_string();
	} else {
		return format!("{}$", wildcard);
	};
}

fn make_name_filter(wildcard: &str) -> String {
	let wildcard = head(wildcard);
	let wildcard = tail(&wildcard);
	let wildcard = wildcard.replace("[", "\\[");
	let wildcard = wildcard.replace("]", "\\]");
	let wildcard = wildcard.replace(".", "\\.");
	let wildcard = wildcard.replace("*", ".+");
	return wildcard;
}

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Settings {
	/// 除外するディレクトリ名
	pub exclude_dirs: Option<std::collections::HashSet<String>>,

	/// 除外するファイル名
	pub exclude_files: Option<std::collections::HashSet<String>>,
}

fn find_settings_toml() -> String {
	// カレントディレクトリを調べます。
	{
		if std::path::Path::new("settings.toml").is_file() {
			return "settings.toml".to_string();
		}
	}

	// ユーザーのホームディレクトリを調べます。(Windows)
	{
		let value = std::env::var("USERPROFILE");
		if value.is_ok() {
			let home = value.unwrap();
			let home = std::path::Path::new(&home);
			let settings_toml = home.join("settings.toml");
			let settings_toml = settings_toml.to_str().unwrap().to_string();
			return settings_toml;
		}
	}

	// ユーザーのホームディレクトリを調べます。(Linux)
	{
		let value = std::env::var("HOME");
		if value.is_ok() {
			let home = value.unwrap();
			let home = std::path::Path::new(&home);
			let settings_toml = home.join("settings.toml");
			let settings_toml = settings_toml.to_str().unwrap().to_string();
			return settings_toml;
		}
	}

	return "".to_string();
}

impl Settings {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `Settings` の新しいインスタンス
	pub fn new() -> Result<Settings, Box<dyn std::error::Error>> {
		let mut instance = Settings {
			exclude_dirs: Some(std::collections::HashSet::new()),
			exclude_files: Some(std::collections::HashSet::new()),
		};

		// 環境に応じた設定ファイルを探します。
		let path = find_settings_toml();

		// コンフィギュレーション
		instance.configure(&path)?;

		return Ok(instance);
	}

	/// コンフィギュレーションを行い、このインスタンスをを初期化します。
	///
	/// # Arguments
	/// * `path` 設定ファイルのパス
	fn configure(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// パスが指定されていなければスキップします。
		if path == "" {
			return Ok(());
		}

		// ファイルが無ければスキップします。
		if !std::path::Path::new(path).is_file() {
			println!("[INFO] Configuration file not found. (settings.toml)");
			return Ok(());
		}

		// テキストファイル全体を読み込み
		let content = functions::read_text_file_all(&path)?;

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
