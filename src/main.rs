//!
//! Entrypoint of application.
//!

mod application;
mod configuration;
mod functions;
mod helpers;
mod util;

/// entrypoint.
fn main() {
	// configure.
	let result = configuration::Settings::new();
	if result.is_err() {
		println!("[ERROR] Configuration error. reason: {}", result.err().unwrap());
		return;
	}
	let settings = result.unwrap();

	// reading commandline options.
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("Path to directory needed.");
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// Stopwatch. For printing summary.
	let stopwatch = util::time::Stopwatch::new();

	// Read the 1st argument.
	let path_to_target = &args[0];

	// Compression.
	let zipper = application::core::Zipper::new();
	let result = zipper.archive(&settings, &path_to_target);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// Summary.
	println!("[INFO] Ok. ({})", stopwatch);

	std::thread::sleep(std::time::Duration::from_millis(900));
}
