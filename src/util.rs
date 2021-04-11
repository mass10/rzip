/// 標準入力から1行読み込みます。終端の改行文字を除く1行全体を返します。
#[allow(unused)]
pub fn read_line() -> String {
	let mut line = String::new();
	let result = std::io::stdin().read_line(&mut line);
	if result.is_err() {
		return String::new();
	}
	return line.trim().to_string();
}

/// エンターキーが押されるまで待機します。
#[allow(unused)]
pub fn pause() {
	let _ = read_line();
}

/// Yes/No で回答すべきプロンプトを表示します。
#[allow(unused)]
pub fn prompt(message: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
	use std::io::Write; // -> Stdout に flush() を実装するトレイト

	println!("{}", message);
	loop {
		print!("(y/N): ");
		std::io::stdout().flush().unwrap();
		let answer = read_line().to_uppercase();
		if answer == "Y" || answer == "YES" {
			return Ok(true);
		}
		if answer == "N" || answer == "NO" {
			return Ok(false);
		}
	}
}

/// タイムスタンプ "%Y-%m-%d %H:%M:%S%.3f" を返します。
#[allow(unused)]
pub fn timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// タイムスタンプ "%Y%m%d-%H%M%S" を返します。
#[allow(unused)]
pub fn timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}

/// ストップウォッチです。
pub struct Stopwatch {
	/// インスタンスが生成された、もしくはオブジェクトがリセットされた日時を指します。
	_time: std::time::Instant,
}

impl Stopwatch {
	/// オブジェクトを生成します。
	pub fn new() -> Stopwatch {
		return Stopwatch { _time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// 経過時間の文字列表現を返します。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use super::helper::MyDurationHelper;
		let elapsed = std::time::Instant::now() - self._time;
		write!(f, "{}", elapsed.to_string2())?;
		return Ok(());
	}
}
