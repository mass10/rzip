//!
//! アプリケーションのエントリーポイントです。
//!

mod application;
mod configuration;
mod functions;
mod helpers;
mod util;

/// アプリケーションのエントリーポイント
fn main() {
	// コマンドライン引数(コマンド自身を除く)
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// 処理時間計測用ストップウォッチ
	let stopwatch = util::time::Stopwatch::new();

	// 第一引数
	let path_to_target = &args[0];

	// コンフィギュレーション
	let result = configuration::Settings::new();
	if result.is_err() {
		println!("[ERROR] Configuration error. reason: {}", result.err().unwrap());
		return;
	}
	let settings = result.unwrap();

	// 書庫化 & ZIP 圧縮
	let zipper = application::core::Zipper::new();
	let result = zipper.archive(&settings, &path_to_target);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// サマリー
	println!("[INFO] Ok. ({})", stopwatch);

	std::thread::sleep(std::time::Duration::from_millis(900));
}
