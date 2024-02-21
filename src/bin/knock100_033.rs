// 整数値を入力させ、1から9まで、入力値以外を表示するプログラム

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  // println!("Input number");
  // let mut input = String::new();
  // std::io::stdin().read_line(&mut input).unwrap();
  // let input: i32 = input.trim().parse().unwrap();
  let input = get_num("input number".to_string());
  for i in 1..10 {
    if i != input {
      print!("{} ", i);
    }
  }
  println!();
}
