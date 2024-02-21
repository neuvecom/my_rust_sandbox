// 整数値を入力させ、0から入力値まで数を1ずつ増やして表示するプログラムを作成せよ。

use std::io::{Write, self};

fn main() {
  print!("input number: ");
  let mut input = String::new();
  io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: u32 = input.trim().parse().unwrap();
  for i in 0..=input {
    println!("{}", i);
  }
}
