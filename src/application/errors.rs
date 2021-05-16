//!
//! アプリケーション定義のエラーを提供しています。
//!

///
/// アプリケーション定義のエラーです。
///
#[derive(std::fmt::Debug, std::clone::Clone)]
pub struct ApplicationError {
	/// メッセージ文字列
	description: String,
}

impl ApplicationError {
	/// 新しいインスタンスを返します。
	///
	/// # Arguments
	/// * `description` エラー文字列
	///
	/// # Returns
	/// `ApplicationError` の新しいインスタンス
	pub fn new(description: &str) -> ApplicationError {
		return ApplicationError { description: description.to_string() };
	}
}

impl std::fmt::Display for ApplicationError {
	/// [std::fmt::Display] としての振る舞いを実装します。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{}", self.description);
	}
}

impl std::error::Error for ApplicationError {
	/// [std::error::Error] としての振る舞いを実装します。
	///
	/// # Returns
	/// エラー文字列
	fn description(&self) -> &str {
		return &self.description;
	}
}
