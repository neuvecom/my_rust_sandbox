// -- モジュールの取り込み
mod lib_knock100_test;
mod lib_knock100_get_num;
// -- ショートカットの定義
use crate::lib_knock100_test::{test, echo_test};
use crate::lib_knock100_get_num::get_num;

fn main() {
  test();
  echo_test("Hello Module!".to_string());
  let num = get_num("input number".to_string());
  println!("input number is: {}", num);
}
