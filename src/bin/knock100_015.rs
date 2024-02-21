// 整数値を入力させ、0から入力値を超えない値まで2ずつ増やして表示するプログラム

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("整数値を入力してください".to_string());
  for i in (0..=input).step_by(2) {
    println!("{}", i);
  }
}
