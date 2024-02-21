/*
整数値を入力させ、その値が5よりも大きく、
かつ、20よりも小さければOKと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("整数値を入力してください".to_string());
  if input > 5 && input < 20 {
    println!("OK");
  }
}
