/*
整数値を入力させ、1からその値までの総和を計算して表示するプログラム
ただし、0以下の値を入力した場合は0と表示する。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  let sum = (1..=input).sum::<i32>();
  println!("{}", if input <= 0 { 0 } else { sum });
}
