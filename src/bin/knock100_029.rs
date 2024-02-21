// 整数値を5回入力させ、それらの値の合計を表示するプログラムを繰り返しを使って作成

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let mut sum = 0;
  for _ in 0..5 {
    let input = get_num("input number".to_string());
    sum += input;
  }
  println!("{}", sum);
}
