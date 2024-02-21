// 整数値を入力させ、0から入力値まで数を1ずつ増やして表示するプログラム

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  for i in 0..=input {
    println!("{}", i);
  }
}
