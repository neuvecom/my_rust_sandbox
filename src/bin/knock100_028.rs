/*
整数値を入力させ、その値の階乗を表示するプログラム
ただし、0以下の値を入力した場合は1と表示する。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  let factorial = (1..=input).product::<i32>();
  println!("{}", if input <= 0 { 1 } else { factorial });
}
