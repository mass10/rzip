[![Build Workflow on GitHub Actions](https://github.com/mass10/rzip/actions/workflows/rust.yml/badge.svg)](https://github.com/mass10/rzip/actions/workflows/rust.yml)

# rzip
 
* 簡易 ZIP アプリケーションです。
* `node_modules` とか `.git` とかを除外できます。

# Getting Started

```COMMAND
cargo install --git https://github.com/mass10/rzip --branch main
rzip path\to\directory
```

# Getting Started (with Source)

```COMMAND
git clone git@github.com:mass10/rzip.git
cargo build --release
target\release\rzip.exe
```
