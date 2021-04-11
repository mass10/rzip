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
		let elapsed = std::time::Instant::now() - self._time;
		write!(f, "{}", format_duration(&elapsed))?;
		return Ok(());
	}
}

/// 経過時間の文字列表現を返します。
fn format_duration(d: &std::time::Duration) -> String {
	let mut millis = d.as_millis();
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
