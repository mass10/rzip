//!
//! 汎用的な各種トレイトの実装
//!

/// [std::fs::DirEntry] のヘルパー
pub trait DirEntityHelper {
	/// パスを `String` で返します。
	///
	/// # Returns
	/// パス文字列
	fn path_as_string(&self) -> String;
}

impl DirEntityHelper for std::fs::DirEntry {
	/// パスを `String` で返します。
	///
	/// # Returns
	/// パス文字列
	fn path_as_string(&self) -> String {
		let tmp = self.path();
		return tmp.to_str().unwrap().to_string();
	}
}

/// [std::path::Path] のヘルパー
pub trait PathHelper {
	/// ファイル名を返します。
	///
	/// # Returns
	/// ファイル名
	fn name_as_str(&self) -> &str;

	/// ファイル名を返します。
	///
	/// # Returns
	/// ファイル名
	fn name_as_string(&self) -> String;

	/// フルパスを返します。
	///
	/// # Returns
	/// フルパス
	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>>;

	fn join_as_string(&self, child: &str) -> Result<String, Box<dyn std::error::Error>>;
}

impl PathHelper for std::path::Path {
	/// ファイル名を返します。
	///
	/// # Returns
	/// ファイル名
	fn name_as_str(&self) -> &str {
		return self.file_name().unwrap().to_str().unwrap();
	}

	/// ファイル名を返します。
	///
	/// # Returns
	/// ファイル名
	fn name_as_string(&self) -> String {
		return self.file_name().unwrap().to_str().unwrap().to_string();
	}

	/// フルパスを返します。
	///
	/// # Returns
	/// フルパス
	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
		let cano = self.canonicalize()?;
		let s = cano.to_str().unwrap().to_string();
		return Ok(s);
	}

	fn join_as_string(&self, child: &str) -> Result<String, Box<dyn std::error::Error>> {
		let result = self.join(child);
		let s = result.to_str().unwrap().to_string();
		return Ok(s);
	}
}

pub trait DurationFormatter {
	/// 経過時間の文字列表現を返します。
	///
	/// # Returns
	/// 文字列
	fn to_string(&self) -> String;
}

impl DurationFormatter for std::time::Duration {
	/// 経過時間の文字列表現を返します。
	///
	/// # Returns
	/// 文字列
	fn to_string(&self) -> String {
		let mut millis = self.as_millis();
		let mut sec = 0;
		let mut min = 0;
		let mut hour = 0;

		while 1000 <= millis {
			sec += 1;
			millis -= 1000;
		}
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}

		let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
		return s;
	}
}
