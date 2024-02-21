/*
整数値を入力させ、その値が1ならone、2ならtwo、3ならthree、
それ以外ならothersと表示するプログラムを
matchを使って作成(swicth-case文から変更)
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  // println!("Input number");
  // let mut input = String::new();
  // std::io::stdin().read_line(&mut input).unwrap();
  // let input: i32 = input.trim().parse().unwrap();
  let input = get_num("input number".to_string());
  match input {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("others"),
  }
}
