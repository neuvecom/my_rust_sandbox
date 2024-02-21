// 整数値を入力させ、その値の回数だけHello World!を繰り返して表示するプログラム

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  for _ in 0..input {
    println!("Hello World!");
  }
}
