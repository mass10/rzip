extern crate serde_derive;

use crate::functions;
#[derive(serde_derive::Deserialize, Debug, std::clone::Clone)]
pub struct Settings {
	exclude_dirs: Option<Vec<String>>,
}

impl Settings {
	/// 新しいインスタンスを返します。
	pub fn new() -> Result<Settings, Box<dyn std::error::Error>> {
		let mut instance = Settings { exclude_dirs: None };
		instance.configure()?;
		return Ok(instance);
	}

	fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let path = "setting.toml";

		if !std::path::Path::new(path).is_file() {
			return Ok(());
		}

		// テキストファイル全体を読み込み
		let content = functions::read_text_file_all(&path)?;

		// toml ファイルをパース
		*self = toml::from_str(&content)?;

		return Ok(());
	}

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
