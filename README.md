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
