# Rust関連の実験用

- [(7) 【Rust コーディング実況解説 #1】FizzBuzz問題をmatch式で美しく](https://www.youtube.com/watch?v=gDyk6OTSdto)

- rustup --version
  - rustup 1.26.0 (5af9b9484 2023-04-05)
  - info: This is the version for the rustup toolchain manager, not the rustc compiler.
  - info: The currently active `rustc` version is `rustc 1.76.0 (07dca489a 2024-02-04)`
- rustc --version
  - rustc 1.76.0 (07dca489a 2024-02-04)
- cargo --version
  - cargo 1.76.0 (c84b36747 2024-01-18)

- cargo init
  - git initもいるか？
- mkdir src/bin
- touch src/bin/fizzbuzz.rs
- cargo run --bin fizzbuzz

- [日付から曜日を計算 ツェラーの公式【Rust コーディング実況解説 #2】 - YouTube](https://www.youtube.com/watch?v=PtuwXqy2LXg)

- touch src/bin/leap_year.rs
- cargo run --bin leap_year
- touch src/bin/zeller.rs
- cargo run --bin zeller

- [【Rust コーディング実況解説 #3】初心に帰って バブルソート - YouTube](https://www.youtube.com/watch?v=69wrkarV0IQ)

- touch src/bin/bubble_sort.rs
  - cargo add rand
- cargo run --bin bubble_sort

- [お勉強: 100本ノック on Rust｜ぐは](https://note.com/densukeo/n/n3a3d734948b0)
  - [基礎プログラミング演習I 〜100本ノック](https://www.cc.kyoto-su.ac.jp/~mmina/bp1/hundredKnocks.html)
  - [densuke/knock100: Rustの勉強として、100本ノックをちまちまとやってみてます。](https://github.com/densuke/knock100)
  - [knock100/src/bin at develop · densuke/knock100](https://github.com/densuke/knock100/tree/develop/src/bin)

基礎プログラミング演習I 〜100本ノックをRustでやってみる
- [基礎プログラミング演習I 〜100本ノック](https://www.cc.kyoto-su.ac.jp/~mmina/bp1/hundredKnocks.html)
- cargo run --bin xxxxxxx
- [基礎プログラミング演習I 〜100本ノック基礎編](https://www.cc.kyoto-su.ac.jp/~mmina/bp1/hundredKnocksBasic.html)
- touch src/bin/knock100_001.rs => 足し算
- touch src/bin/knock100_002.rs => 余り
- touch src/bin/knock100_003.rs => 入力
- touch src/bin/knock100_004.rs => 入力と計算
- touch src/bin/knock100_004.rs => 四則演算
- ...

- [なぜかRustで言語処理100本ノック ～目次～ #Rust - Qiita](https://qiita.com/kbone/items/e15583bf5084c45c5c82)
  - [言語処理100本ノック 2015](https://www.cl.ecei.tohoku.ac.jp/nlp100/)
    - [なぜかRustで言語処理100本ノック ～第1章 前編～ #Rust - Qiita](https://qiita.com/kbone/items/47d108d361359de6b34c)
- [ナコちゃんはRust言語のチュートリアルをやるそうです。 - YouTube](https://www.youtube.com/playlist?list=PLZC0EFxamvoA4jZYO8Az1BKizddQ3aDp3)
- [Rust 練習用](https://gist.github.com/shootacean/0907b8613b4b164f8196599ee871c564)

## メモ
モチベが下がるので、ファイル生成ツール(setup.sh)を作った
src/binの中にメインのあるファイルを置くと良いらしい
