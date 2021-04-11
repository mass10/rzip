mod application;
mod application_error;
mod configuration;

/// アプリケーションのエントリーポイント
fn main() {
	// コンフィギュレーション
	let result = configuration::ConfigurationSettings::configure();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let _conf = result.unwrap();

	// アプリケーションのインスタンスを初期化します。
	let result = application::Application::new();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
	let app = result.unwrap();

	// アプリケーションを実行します。
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
