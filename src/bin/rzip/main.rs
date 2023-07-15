//!
//! Entrypoint of application.
//!

mod application;
mod configuration;
mod util;

/// Show usage.
fn usage(options: &getopts::Options) {
	eprintln!("RZIP: Recursively compress files and directories.");
	eprintln!("    * Recursively compresses all files and directories under the specified path.");
	eprintln!("    * If the specified path is a directory, the directory name is used as the root of the archive.");
	eprintln!();
	eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
	eprint!("{}", options.usage(""));
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
	options.opt("h", "help", "Usage.", "", getopts::HasArg::No, getopts::Occur::Optional);
	options.opt("", "root", "No root folder.", "", getopts::HasArg::No, getopts::Occur::Optional);
	options.opt("", "sleep", "Sleep {n} seconds after finish.", "NUMBER", getopts::HasArg::Yes, getopts::Occur::Optional);
	let result = options.parse(args);
	if result.is_err() {
		eprintln!("{}", result.err().unwrap());
		eprintln!();
		usage(&options);
		std::process::exit(1);
	}
	let matches = result.unwrap();

	if matches.opt_present("h") {
		usage(&options);
		return;
	}

	// Sleep after finish.
	let sleep_seconds: f32 = if matches.opt_present("sleep") {
		let value = matches.opt_str("sleep").unwrap();
		let value = value.parse::<f32>();
		if value.is_err() {
			eprintln!("Invalid value for option: '--sleep'");
			eprintln!();
			usage(&options);
			std::process::exit(1);
		}
		value.unwrap()
	} else {
		0.0
	};
	let milliseconds = sleep_seconds * 1000.0;
	let milliseconds = milliseconds as u64;

	// Optional: No root directory.
	let create_root = !matches.opt_present("root");

	// Free options.
	let free_args = matches.free;

	if free_args.len() < 2 {
		usage(&options);
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
	let result = zipper.archive(&settings, &path_to_archive, &path_to_source, create_root);
	if result.is_err() {
		eprintln!("[ERROR] Runtime error. reason: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_millis(milliseconds));
		std::process::exit(1);
	}

	// Summary.
	println!("[INFO] Ok. ({})", stopwatch);
	std::thread::sleep(std::time::Duration::from_millis(milliseconds));
}
