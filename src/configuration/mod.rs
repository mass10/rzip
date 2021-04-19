extern crate serde_derive;

use crate::functions;

#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Settings {
	/// 除外するディレクトリ名
	pub exclude_dirs: Option<Vec<String>>,
	/// 除外するファイル名
	pub exclude_files: Option<std::collections::HashSet<String>>,
}

impl Settings {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// `Settings` の新しいインスタンス
	pub fn new() -> Result<Settings, Box<dyn std::error::Error>> {
		let mut instance = Settings {
			exclude_dirs: Some(vec![]),
			exclude_files: Some(std::collections::HashSet::new()),
		};
		instance.configure("settings.toml")?;
		return Ok(instance);
	}

	/// コンフィギュレーションを行い、このインスタンスをを初期化します。
	///
	/// # Arguments
	/// * `path` 設定ファイルのパス
	fn configure(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// ファイルが無ければスキップします。
		if !std::path::Path::new(path).is_file() {
			println!("[TRACE] Configuration file not found. (settings.toml)");
			return Ok(());
		}
		// テキストファイル全体を読み込み
		let content = functions::read_text_file_all(&path)?;
		// toml ファイルをパース
		*self = toml::from_str(&content)?;
		if self.exclude_dirs.is_none() {
			self.exclude_dirs = Some(vec![]);
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
		let dirs = self.exclude_dirs.as_ref().unwrap();
		for e in dirs {
			if name == e {
				return false;
			}
		}
		return true;
	}
}
