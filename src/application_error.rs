/// アプリケーション定義エラー
#[derive(Debug, Clone)]
pub struct ApplicationError {
	/// メッセージ文字列
	description: String,
}

impl ApplicationError {
	/// 新しいインスタンスを返します。
	pub fn new(description: &str) -> ApplicationError {
		return ApplicationError { description: description.to_string() };
	}
}

/// [std::fmt::Display] としての振る舞いを実装します。
impl std::fmt::Display for ApplicationError {
	/// 規定の操作をインプリメントします。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		return write!(f, "{}", self.description);
	}
}

/// [std::error::Error] としての振る舞いを実装します。
impl std::error::Error for ApplicationError {
	/// 規定の操作をインプリメントします。
	fn description(&self) -> &str {
		return &self.description;
	}
}
