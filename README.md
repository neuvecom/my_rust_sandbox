# 個人的なRustの実験用

[【Rust コーディング実況解説 #1】FizzBuzz問題をmatch式で美しく](https://www.youtube.com/watch?v=gDyk6OTSdto)

Rustの環境を確認
- `rustup --version`
  - rustup 1.26.0 (5af9b9484 2023-04-05)
  - info: This is the version for the rustup toolchain manager, not the rustc compiler.
  - info: The currently active `rustc` version is `rustc 1.76.0 (07dca489a 2024-02-04)`
- `rustc --version`
  - rustc 1.76.0 (07dca489a 2024-02-04)
- `cargo --version`
  - cargo 1.76.0 (c84b36747 2024-01-18)

プロジェクトディレクトリの用意
- `cargo init` => 既存のディレクトリの時は、このコマンドで
  - `git init`もいるか？ => 上記コマンドで一緒に処理される模様
- `mkdir src/bin` => src/bin で個別にmainを持つファイルが使える
- `touch src/bin/fizzbuzz.rs` => ファイルの生成
- `cargo run --bin fizzbuzz` => 個別のファイルの実行コマンド

[日付から曜日を計算 ツェラーの公式【Rust コーディング実況解説 #2】 - YouTube](https://www.youtube.com/watch?v=PtuwXqy2LXg)

ファイルを作成
- `touch src/bin/leap_year.rs`
- `cargo run --bin leap_year`
- `touch src/bin/zeller.rs`
- `cargo run --bin zeller`

[【Rust コーディング実況解説 #3】初心に帰って バブルソート - YouTube](https://www.youtube.com/watch?v=69wrkarV0IQ)

ファイルを作成
- `touch src/bin/bubble_sort.rs`
  - `cargo add rand`
- `cargo run --bin bubble_sort`

基礎プログラミング演習I 〜100本ノックをRustでやってみる
- [基礎プログラミング演習I 〜100本ノック](https://www.cc.kyoto-su.ac.jp/~mmina/bp1/hundredKnocks.html)
- [進捗](./src/bin/knock100.md)
- `zsh setup.sh`
- `zsh run.sh`
- [Rustの100本ノックを受けようとしているけど、なかなか進まない件](https://zenn.dev/neuvecom/articles/bd97773d971a33)

参考になるURL
- [お勉強: 100本ノック on Rust｜ぐは](https://note.com/densukeo/n/n3a3d734948b0)
- [densuke/knock100: Rustの勉強として、100本ノックをちまちまとやってみてます。](https://github.com/densuke/knock100)
- [knock100/src/bin at develop · densuke/knock100](https://github.com/densuke/knock100/tree/develop/src/bin)

その他の資料
- [なぜかRustで言語処理100本ノック ～目次～ #Rust - Qiita](https://qiita.com/kbone/items/e15583bf5084c45c5c82)
  - [言語処理100本ノック 2015](https://www.cl.ecei.tohoku.ac.jp/nlp100/)
    - [なぜかRustで言語処理100本ノック ～第1章 前編～ #Rust - Qiita](https://qiita.com/kbone/items/47d108d361359de6b34c)
- [ナコちゃんはRust言語のチュートリアルをやるそうです。 - YouTube](https://www.youtube.com/playlist?list=PLZC0EFxamvoA4jZYO8Az1BKizddQ3aDp3)
- [Rust 練習用](https://gist.github.com/shootacean/0907b8613b4b164f8196599ee871c564)

## メモ
- モチベが下がるので、ファイル生成ツール(setup.sh)を作った
  - src/binの中にメインのあるファイルを置くと良いらしい
  - run.shも作った

## 一段落したら読む
- [Rustの実用性が理解できる図を作成してみた 〜C/C++/Java/JS/Python/Go/TS/Elixirとの比較〜](https://zenn.dev/hinastory/articles/7857427ea390c5)
- [Actix非公式 日本語ドキュメント](https://actix-website.pages.dev/)
- [Unsafeがよくわからなかったので整理した (Rust) - 射撃しつつ前進](https://gkuga.hatenablog.com/entry/2020/02/14/132244)
- [マクロに入門したので概要を書く (Rust) - 射撃しつつ前進](https://gkuga.hatenablog.com/entry/2020/02/07/225353)

## 一時的にメモ
- [RustでGTFSファイルをURLから読み込む #Rust - Qiita](https://qiita.com/awrznc/items/3b0b71895e0221b25c93)
  - [GTFSってなんですか？ | 地域公共交通のトリセツ（取組説明書）](https://kotsutorisetsu.com/20201105-2/)
- [ファイル読み込み \[File, BufReader\] Rust Tips-モバイル開発系(K)](https://www.office-matsunaga.biz/rust/description.php?id=9)