mod application;
mod application_error;
mod configuration;
mod helper;
mod util;

/// アプリケーションのエントリーポイント
fn main() {
	// コンフィギュレーション
	if false {
		let result = configuration::ConfigurationSettings::configure();
		if result.is_err() {
			println!("[ERROR] Configuration error. reason: {}", result.err().unwrap());
			return;
		}
		let _conf = result.unwrap();
	}

	// コマンドライン引数(コマンド自身を除く)
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// 処理時間計測用ストップウォッチ
	let stopwatch = util::Stopwatch::new();

	// 第一引数
	let path_to_target = &args[0];

	// 書庫化 & ZIP 圧縮
	let app = application::Application::new();
	let result = app.archive(&path_to_target);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// サマリー
	println!("[INFO] Ok. ({})", stopwatch);

	std::thread::sleep(std::time::Duration::from_secs(2));
}
