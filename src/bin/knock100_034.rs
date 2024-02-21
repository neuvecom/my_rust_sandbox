/*
整数値を入力させ、1から9まで、
入力値と入力値+1以外を表示するプログラム
入力値が9の場合は9のみ表示しない。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  for i in 1..10 {
    if i != input && i != input + 1 {
      print!("{} ", i);
    }
  }
  println!();
}
