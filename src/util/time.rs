//!
//! Operations for date and time.
//!

///
/// Stopwatch
///
pub struct Stopwatch {
	/// remains current timestamp.
	time: std::time::Instant,
}

impl Stopwatch {
	/// Returns a new instance of `Stopwatch`.
	///
	/// ### Returns
	/// A new instance of `Stopwatch`.
	pub fn new() -> Stopwatch {
		return Stopwatch { time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// Implements default behavior as [std::fmt::Display].
	///
	/// ### Returns
	/// Duration as formatted string.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use crate::helpers::DurationFormatter;

		let elapsed = std::time::Instant::now() - self.time;
		write!(f, "{}", elapsed.to_string())?;
		return Ok(());
	}
}
