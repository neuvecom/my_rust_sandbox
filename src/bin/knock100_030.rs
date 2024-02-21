/*
整数値を入力させ、その個数だけ*を表示するプログラム
入力値が0以下の値の場合は何も書かなくてよい。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  if input > 0 {
    println!("{}", "*".repeat(input as usize));
  }
}
