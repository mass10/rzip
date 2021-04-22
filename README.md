[![Build Workflow on GitHub Actions](https://github.com/mass10/rzip/actions/workflows/rust.yml/badge.svg)](https://github.com/mass10/rzip/actions/workflows/rust.yml)

# About
 
* Simple ZIP-per.
* Able to exclude something like `node_modules`, `.git`.

# Getting Started (with Source)

* Modify to implement your own filter.

```COMMAND
git clone https://github.com/mass10/rzip
cargo build --release
target\release\rzip.exe
```

# Getting Started (with Binary)

* Not recomended unless you understand what you're doing.

```COMMAND
cargo install --git https://github.com/mass10/rzip --branch main
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

* Enable to excluding filters on your own settings.toml. (WIP)
