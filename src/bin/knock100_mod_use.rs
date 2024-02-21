// -- モジュールの取り込み
mod lib_knock100;
// -- ショートカットの定義
use crate::lib_knock100::{test, echo_test, get_num};
/* ショットカットは使用モジュールがわかるようなのにしたほうがいいかも */

fn main() {
  test();
  echo_test("Hello Module!".to_string());
  let num = get_num("input number".to_string());
  println!("input number is: {}", num);
}
