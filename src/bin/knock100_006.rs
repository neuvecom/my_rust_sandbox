// 整数値を入力させ、値が0ならzeroと表示するプログラム。
// 整数値を入力させ、値が0ならzero、0でなければnot zeroと表示するプログラムを作成せよ。
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
    if output == 0 {
        println!("The input number is zero.");
    } else {
        println!("{} is not zero.", output);
    }
}
