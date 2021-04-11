/// std::time::Duration にアプリケーション固有のフォーマット操作を定義しています。
pub trait MyDurationHelper {
	/// 経過時間の文字列表現を得る
	fn to_string1(&self) -> String;
	/// 経過時間の文字列表現を得る
	fn to_string2(&self) -> String;
}

impl MyDurationHelper for std::time::Duration {
	fn to_string1(&self) -> String {
		let mut sec = self.as_secs();
		let mut min = 0;
		let mut hour = 0;
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		let s = format!("{:02}:{:02}:{:02}", hour, min, sec);
		return s;
	}

	fn to_string2(&self) -> String {
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
