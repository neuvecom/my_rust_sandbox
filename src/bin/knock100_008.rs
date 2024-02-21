/*
整数値を入力させ、値が正であればpositiveと表示するプログラムを作成せよ。ただし0は正には含まない。
整数値を入力させ、値が正であればpositive、負であればnegative、0であればzeroと表示するプログラムを作成せよ。
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
    if output > 0 {
        println!("{} is positive", output);
    } else if output < 0 {
        println!("{} is negative.", output);
    } else {
        println!("The input number is zero.");
    }
}
