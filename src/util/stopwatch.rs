/// ストップウォッチです。
pub struct Stopwatch {
	/// インスタンスが生成された、もしくはオブジェクトがリセットされた日時を指します。
	time: std::time::Instant,
}

impl Stopwatch {
	/// オブジェクトを生成します。
	pub fn new() -> Stopwatch {
		return Stopwatch { time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// 経過時間の文字列表現を返します。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use super::helpers::DurationFormatter;

		let elapsed = std::time::Instant::now() - self.time;
		write!(f, "{}", elapsed.to_string())?;
		return Ok(());
	}
}
