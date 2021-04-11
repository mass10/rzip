use crate::util::helpers::PathHelper;

use super::super::util::functions;
use super::super::util::helpers;
use super::errors::ApplicationError;
use std::io::Read;

/// 正規表現による文字列のマッチング
fn matches(pattern: &str, text: &str) -> bool {
	let reg = regex::Regex::new(pattern);
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

/// ファイル名の検証
pub fn is_valid_file(path: &std::path::Path) -> bool {
	let name = path.file_name().unwrap().to_str().unwrap();
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches(".VC.db$", name) {
		return false;
	}
	// で終わる
	if matches(".ipch$", name) {
		return false;
	}
	return true;
}

/// ディレクトリの妥当性を検証します。
pub fn is_valid_directory(dir: &std::path::Path) -> bool {
	match dir.file_name().unwrap().to_str().unwrap() {
		"node_modules" => return false,
		".git" => return false,
		"dist" => return false,
		".nuxt" => return false,
		"Debug" => return false,
		"Release" => return false,
		"ReleaseDebug" => return false,
		"target" => return false,
		"ipch" => return false,
		"x64" => return false,
		_ => {}
	};
	return true;
}

/// アプリケーション本体の定義
pub struct Zipper;

impl Zipper {
	/// 新しいインスタンスを返します。
	///
	/// # Returns
	/// 新しいアプリケーションのインスタンス
	pub fn new() -> Zipper {
		let instance = Zipper {};
		return instance;
	}

	/// アーカイバーにエントリーを追加します。
	fn append_entry(&self, archiver: &mut zip::ZipWriter<std::fs::File>, base_name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		use helpers::DirEntityHelper;
		use std::io::Write;

		let unknown = std::path::Path::new(path);
		if unknown.is_dir() {
			// ディレクトリの名前を検査しています。
			// TODO: 廃止予定
			if !is_valid_directory(unknown) {
				return Ok(());
			}

			// ディレクトリ名
			let name = unknown.name_as_str();
			// ZIP ルートからの相対パス
			let internal_path = functions::build_path(base_name, name);
			// 内部構造にディレクトリエントリーを作成(二段目以降)
			if base_name != "" {
				println!("adding file ... {}", &base_name);
				let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
				archiver.add_directory(&internal_path, options)?;
			}
			// サブディレクトリを走査
			let it = std::fs::read_dir(path)?;
			for e in it {
				let entry = e?;
				let fullpath = entry.path_as_string();
				self.append_entry(archiver, &internal_path, &fullpath)?;
			}
		} else if unknown.is_file() {
			// ディレクトリの名前を検査しています。
			// TODO: 廃止予定
			if !is_valid_file(unknown) {
				return Ok(());
			}

			// ファイル名
			let name = unknown.name_as_str();
			// ZIP ルートからの相対パス
			let relative_path = functions::build_path(base_name, name);
			// ファイルのメタ情報
			let meta = unknown.metadata()?;
			// ファイルをアーカイブ
			let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
			// 最終更新日時
			let last_modified = meta.modified()?;
			let last_modified = functions::convert_datetime0(last_modified);
			let options = options.last_modified_time(last_modified);

			// 内部構造にファイルエントリーを作成
			println!("adding file ... {}", &relative_path);
			archiver.start_file(&relative_path, options)?;
			let mut stream = std::fs::File::open(path)?;
			loop {
				let mut buffer = [0; 1000];
				let bytes_read = stream.read(&mut buffer)?;
				if bytes_read == 0 {
					break;
				}
				let write_buffer = &buffer[..bytes_read];
				archiver.write(&write_buffer)?;
			}
		} else {
			let message = format!("Unknown filesystem [{}].", path);
			let err = ApplicationError::new(&message);
			return Err(Box::new(err));
		}

		return Ok(());
	}

	/// ディレクトリをアーカイブします。
	///
	/// # Arguments
	/// `path` パス
	pub fn archive(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// パスを正規化
		let path = functions::canonicalize_path(path)?;

		println!("archiving ... {}", &path);

		// ファイル名を生成
		let archive_path_name = format!("{}.zip", &path);

		// 起点となるディレクトリの名前
		let base_name = "";

		// .zip ファイルがあれば削除
		functions::unlink(&archive_path_name)?;

		// アーカイバーの初期化
		let w = std::fs::File::create(archive_path_name)?;
		let mut archiver = zip::ZipWriter::new(w);

		// ここから走査
		self.append_entry(&mut archiver, &base_name, &path)?;

		// アーカイバーを閉じます。
		archiver.finish()?;

		return Ok(());
	}
}
