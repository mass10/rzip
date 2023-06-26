//!
//! Entrypoint of application.
//!

mod application;
mod configuration;
mod functions;
mod helpers;
mod util;

/// Show usage.
fn show_usage_header() {
	eprintln!("RZIP: Recursively compress files and directories.");
	eprintln!("    * Recursively compresses all files and directories under the specified path.");
	eprintln!("    * If the specified path is a directory, the directory name is used as the root of the archive.");
}

/// Entrypoint.
fn main() {
	// Configure.
	let result = configuration::Settings::new();
	if result.is_err() {
		eprintln!("[ERROR] Configuration error. reason: {}", result.err().unwrap());
		std::process::exit(1);
	}
	let settings = result.unwrap();

	// Commandline options.
	let args: Vec<String> = std::env::args().skip(1).collect();

	// Parse commandline options.
	let mut options = getopts::Options::new();
	options.opt("h", "help", "Usage.", "FLAG", getopts::HasArg::No, getopts::Occur::Optional);
	options.opt("", "root", "No root folder.", "FLAG", getopts::HasArg::No, getopts::Occur::Optional);
	options.opt("", "wait", "Finish with sleep {n} seconds.", "NUMBER", getopts::HasArg::Yes, getopts::Occur::Optional);
	let result = options.parse(args);
	if result.is_err() {
		eprintln!("{}", result.err().unwrap());
		eprintln!();

		// Usage.
		show_usage_header();
		eprintln!();
		eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
		eprint!("{}", options.usage(""));

		std::process::exit(1);
	}
	let matches = result.unwrap();

	if matches.opt_present("h") {
		// Usage.
		show_usage_header();
		eprintln!();
		eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
		eprint!("{}", options.usage(""));

		return;
	}

	// Wait after finish.
	let wait_seconds: f32 = if matches.opt_present("wait") {
		let value = matches.opt_str("wait").unwrap();
		let value = value.parse::<f32>();
		if value.is_err() {
			eprintln!("[ERROR] Invalid value for -W option.");

			// Usage.
			show_usage_header();
			eprintln!();
			eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
			eprint!("{}", options.usage(""));

			std::process::exit(1);
		}
		value.unwrap()
	} else {
		0.0
	};
	let milliseconds = wait_seconds * 1000.0;
	let milliseconds = milliseconds as u64;

	// Free options.
	let free_args = matches.free;

	if free_args.len() < 2 {
		// Usage.
		show_usage_header();
		eprintln!();
		eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
		eprint!("{}", options.usage(""));

		std::process::exit(1);
	}

	// Stopwatch. For printing summary.
	let stopwatch = util::Stopwatch::new();

	// 1st argument is path to archive.
	let path_to_archive = &free_args[0];

	// 2nd argument is path to file or directory.
	let path_to_source = &free_args[1];

	// Compression.
	let zipper = application::Zipper::new();
	let result = zipper.archive(&settings, &path_to_archive, &path_to_source);
	if result.is_err() {
		eprintln!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_millis(milliseconds));
		std::process::exit(1);
	}

	// Summary.
	println!("[INFO] Ok. ({})", stopwatch);
	std::thread::sleep(std::time::Duration::from_millis(milliseconds));
}
