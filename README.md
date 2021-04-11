[![Build Workflow on GitHub Actions](https://github.com/mass10/rzip/actions/workflows/rust.yml/badge.svg)](https://github.com/mass10/rzip/actions/workflows/rust.yml)

# rzip
 
* 簡易 ZIP アプリケーションです。
* `node_modules` とか `.git` とかを除外できます。

# Getting Started (with Source)

* ソースからビルドし、独自のファイル名検査を追加・修正してください。

```COMMAND
git clone git@github.com:mass10/rzip.git
cargo build --release
target\release\rzip.exe
```

# Getting Started (with Binary)

* 作者固有のファイル名検査が入っているため、おすすめしません。

```COMMAND
cargo install --git https://github.com/mass10/rzip --branch main
rzip path\to\directory
```

# Future Plans

* ファイル名検査を toml で定義できるようにしたい。
