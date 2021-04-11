pub trait MyHelper {
	fn path_as_string(&self) -> String;
}

impl MyHelper for std::fs::DirEntry {
	fn path_as_string(&self) -> String {
		let tmp = self.path();
		return tmp.to_str().unwrap().to_string();
	}
}

pub trait PathHelper {
	fn name_as_str(&self) -> &str;
	fn name_as_string(&self) -> String;
	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>>;
}

impl PathHelper for std::path::Path {
	fn name_as_str(&self) -> &str {
		return self.file_name().unwrap().to_str().unwrap();
	}

	fn name_as_string(&self) -> String {
		return self.file_name().unwrap().to_str().unwrap().to_string();
	}

	fn canonical_path_as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
		let cano = self.canonicalize()?;
		let s = cano.to_str().unwrap().to_string();
		return Ok(s);
	}
}
