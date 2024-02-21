// -- モジュールの取り込み
mod lib_knock100;

fn main() {
  lib_knock100::test();
  lib_knock100::echo_test("Hello Module!".to_string());
  let num = lib_knock100::get_num("input number".to_string());
  println!("input number is: {}", num);
}
