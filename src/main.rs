//!
//! Entrypoint of application.
//!

mod application;
mod configuration;
mod functions;
mod helpers;
mod util;

/// Show usage.
fn show_usage() {
	eprintln!("Usage: rzip \"new.zip\" \"path to archive\"");
}

/// Entrypoint.
fn main() {
	// configure.
	let result = configuration::Settings::new();
	if result.is_err() {
		eprintln!("[ERROR] Configuration error. reason: {}", result.err().unwrap());
		std::process::exit(1);
	}
	let settings = result.unwrap();

	// reading commandline options.
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() < 1 {
		show_usage();
		std::process::exit(1);
	}

	for arg in &args {
		if arg == "-h" || arg == "--help" {
			show_usage();
			return;
		}
		// Recursively in default.
		// else if arg == "-r" || arg == "--recursive" {
		// }
	}

	if args.len() < 2 {
		show_usage();
		std::process::exit(1);
	}

	// Stopwatch. For printing summary.
	let stopwatch = util::time::Stopwatch::new();

	// 1st argument is path to archive.
	let path_to_archive = &args[0];

	// 2nd argument is path to file or directory.
	let path_to_source = &args[1];

	// Compression.
	let zipper = application::core::Zipper::new();
	let result = zipper.archive(&settings, &path_to_archive, &path_to_source);
	if result.is_err() {
		println!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(3));
		return;
	}

	// Summary.
	println!("[INFO] Ok. ({})", stopwatch);

	std::thread::sleep(std::time::Duration::from_millis(900));
}
