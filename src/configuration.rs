use super::application_error;

/// コンフィギュレーション構造体
#[derive(Debug)]
pub struct ConfigurationSettings {
	/// ポート番号
	pub port: u32,
	/// サーバー
	pub server: String,
}

impl ConfigurationSettings {
	/// 新しいインスタンスを返します。
	pub fn configure() -> Result<ConfigurationSettings, std::boxed::Box<dyn std::error::Error>> {
		// コンフィギュレーション構造体
		let mut conf = ConfigurationSettings { port: 0, server: "".to_string() };

		// 提供するコマンドラインオプションを定義しています。
		let mut getopt = getopts::Options::new();
		getopt.optflag("h", "help", "");
		getopt.optopt("s", "server", "", "");
		getopt.optopt("p", "port", "", "");

		// 解析
		let result = getopt.parse(std::env::args().skip(1));
		if result.is_err() {
			println!("{}", result.err().unwrap());
			println!("{}", getopt.usage(""));
			return Err(Box::new(application_error::ApplicationError::new("")));
		}
		let result = result.unwrap();

		// help
		if result.opt_present("help") {
			println!("{}", getopt.usage(""));
			return Err(Box::new(application_error::ApplicationError::new("")));
		}

		// server
		if result.opt_present("server") {
			conf.server = result.opt_str("server").unwrap();
		}

		// port
		if result.opt_present("port") {
			let result = result.opt_str("port").unwrap().parse();
			if result.is_err() {
				println!("{}", getopt.usage(""));
				return Err(Box::new(application_error::ApplicationError::new("")));
			}
			conf.port = result.unwrap();
		}

		return Ok(conf);
	}
}

/// [std::fmt::Display] としての振る舞いを実装します。
impl std::fmt::Display for ConfigurationSettings {
	/// 規定の操作をインプリメントします。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{{ server: \"{}\", port: {} }}", self.server, self.port);
	}
}
