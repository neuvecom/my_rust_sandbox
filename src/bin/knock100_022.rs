/*
整数値を入力させ、その値が-10以下、または、10以上であれば
OKと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("整数値を入力してください".to_string());
  if input <= -10 || input >= 10 {
    println!("OK");
  }
}
