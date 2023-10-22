//!
//! Entrypoint of application.
//!

mod application;
mod archiver;
mod configuration;
mod util;

struct CommandlineOptions {
	// Core options.
	options: getopts::Options,

	/// Option: --help
	pub help: bool,

	/// Option: --root
	pub root: bool,

	/// Option: --sleep
	pub sleep: Option<f32>,

	/// Others.
	pub free: Vec<String>,
}

impl CommandlineOptions {
	/// Create new instance.
	pub fn new() -> Self {
		let mut options = getopts::Options::new();
		options.opt("h", "help", "Usage.", "", getopts::HasArg::No, getopts::Occur::Optional);
		options.opt("", "root", "No root folder.", "", getopts::HasArg::No, getopts::Occur::Optional);
		options.opt(
			"",
			"sleep",
			"Sleep {n} seconds after finish.",
			"NUMBER",
			getopts::HasArg::Yes,
			getopts::Occur::Optional,
		);

		let instance = CommandlineOptions {
			options: options,
			help: false,
			root: false,
			sleep: None,
			free: Vec::new(),
		};

		return instance;
	}

	/// Parse commandline options.
	pub fn parse(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
		// Commandline options.
		let args: Vec<String> = std::env::args().skip(1).collect();

		let matches = self.options.parse(args)?;

		// Option: --help
		self.help = matches.opt_present("help");

		// Option: --root
		self.root = matches.opt_present("root");

		// Option: --sleep
		if matches.opt_present("sleep") {
			let value = matches.opt_str("sleep").unwrap();
			let value = value.parse::<f32>();
			if value.is_err() {
				return Err("Invalid value for option: '--sleep'".into());
			}
			self.sleep = Some(value.unwrap());
		};

		// Free options.
		self.free = matches.free;

		return Ok(());
	}

	pub fn free(&self) -> &Vec<String> {
		return &self.free;
	}

	/// Show usage.
	pub fn usage(&self) {
		let options = &self.options;

		eprintln!("RZIP: Recursively compress files and directories.");
		eprintln!("    * Recursively compresses all files and directories under the specified path.");
		eprintln!("    * If the specified path is a directory, the directory name is used as the root of the archive.");
		eprintln!();
		eprint!("{}", options.short_usage("\n    rzip \"archived.zip\" \"path to archive\""));
		eprint!("{}", options.usage(""));
	}
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

	// Parse commandline options.
	let mut options = CommandlineOptions::new();
	let result = options.parse();
	if result.is_err() {
		eprintln!("{}", result.err().unwrap());
		eprintln!();
		options.usage();
		std::process::exit(1);
	}

	// Show usage.
	if options.help {
		options.usage();
		return;
	}

	// Sleep after finish.
	let milliseconds = (options.sleep.unwrap_or(0.0) * 1000.0) as u64;

	// Optional: No root directory.
	let create_root = !options.root;

	// Free options.
	let free_args = options.free();

	if free_args.len() < 2 {
		options.usage();
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
