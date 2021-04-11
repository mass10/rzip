/// アプリケーション構造体
pub struct Application;

impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> std::result::Result<Application, std::boxed::Box<dyn std::error::Error>> {
		return Ok(Application {});
	}

	/// アプリケーションを実行します。
	pub fn run(&self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		return Ok(());
	}
}
