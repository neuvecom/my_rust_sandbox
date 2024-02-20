// 整数値を入力させ、その入力値を3倍した計算結果を表示するプログラムを作成せよ。
use std::io::{Write, self};

fn main() {
    let mut input = String::new();
    print!("input number:");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let output: i32 = input.parse().unwrap();
    println!("your number x 3 is: {}", output * 3);
}

