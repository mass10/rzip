# About
 
* Simple ZIP-per.
* Able to exclude something like `node_modules`, `.git` with settings.toml.
# Getting Started

```CMD
cargo install rzip

rzip archive.zip path\to\directory
```

# settings.toml

```toml
exclude_dirs = [
	".git",
	".settings"
]

exclude_files = [
	"*.vcxproj.user",
	"*.obj"
]
```

# Examples

```CMD
REM Extracts Source-20230624-184501.zip
rzip.exe "{0}-{Y}{m}{d}-{H}{M}{S}.zip" C:\Path\To\Source
```

> Source-

```sh
# Extracts source-20230624-184501.zip
rzip "%0-%Y%m%d-%H%M%S.zip" /path/to/source
```


# Future Plans

* Improve recognizing settings.toml.
* Recognizing ~/.rziprc
