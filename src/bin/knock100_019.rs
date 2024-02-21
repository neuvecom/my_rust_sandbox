/*
要素数5の整数型の配列を宣言し、
すべての配列に対して順に入力された整数値を代入し、
すべての要素の値を表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let mut array: [i32; 5] = [0; 5];
  for i in 0..5 {
    // println!("整数値を入力してください。");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input: i32 = input.trim().parse().unwrap();
    let input = get_num("整数値を入力してください".to_string());
    array[i] = input;
  }
  println!("{:?}", array);
  // for i in 0..5 {
  //   println!("{}", array[i]);
  // }
}
