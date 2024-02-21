// -- モジュールの取り込み
mod lib_knock100_test;
mod lib_knock100_get_num;

fn main() {
  lib_knock100_test::test();
  lib_knock100_test::echo_test("Hello Module!".to_string());
  let num = lib_knock100_get_num::get_num("input number".to_string());
  println!("input number is: {}", num);
}
