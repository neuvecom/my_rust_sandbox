/*
整数値を入力させ、その値を絶対値にして表示するプログラムを作成せよ。
（できれば変数の値を絶対値に変えるようにせよ）
*/

use std::io::{Write, self};

fn main() {
    // 初期化
    let mut input = String::new();
    // 入力
    print!("input number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let output: i32 = input.parse().unwrap();
    // 結果を出力
    println!("absolute value is {}", output.abs());
}
