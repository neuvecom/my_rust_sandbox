// 整数値を入力させ、入力値から0まで数を1ずつ減らして表示するプログラムを作成せよ。

use std::io::{Write, self};

fn main() {
  print!("input number: ");
  let mut input = String::new();
  io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: u32 = input.trim().parse().unwrap();
  for i in (0..=input).rev() {
    println!("{}", i);
  }
}
