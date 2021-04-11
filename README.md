![Rust のアプリケーションをビルドするワークフロー](https://github.com/mass10/easy-archiver-0/workflows/Rust%20%E3%81%AE%E3%82%A2%E3%83%97%E3%83%AA%E3%82%B1%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3%E3%82%92%E3%83%93%E3%83%AB%E3%83%89%E3%81%99%E3%82%8B%E3%83%AF%E3%83%BC%E3%82%AF%E3%83%95%E3%83%AD%E3%83%BC/badge.svg)

# easy-archiver-0
 
* 7-Zip の力を仮りてガッと zip するやつ
* `node_modules` とか `.git` とかを除外できる

# ビルドするには

rustup を公式サイトから持ってきてインストールしたら

```COMMAND
make.bat
```

* `cargo run` で実行する場合は、あらかじめビルドする必要はありません。ビルドすると、ビルドされたプラットフォームで直接実行ができるバイナリが生成されます。

# バックアップするには

```COMMAND
zip.bat "C:\\path\\to\\directory"
```

* cargo run してるので、あらかじめビルドしておく必要はありません。必要に応じて、即座にビルドが走ります。
