/*
整数値を入力させ、その個数だけ*を、
5個おきに空白（スペース）を入れて表示するプログラム
入力値が0以下の値の場合は何も書かなくてよい。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number".to_string());
  if input > 0 {
    for i in 1..input {
      if i % 5 == 0 {
        print!("* ");
      } else {
        print!("*");
      }
    }
    println!(" ");
  }
}
