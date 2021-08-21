# About
 
* Simple ZIP-per.
* Able to exclude something like `node_modules`, `.git`.
# Getting Started

```COMMAND
cargo install rzip
rzip path\to\directory
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

# Future Plans

* Improve recognizing settings.toml.
* Recognizing ~/.rziprc
