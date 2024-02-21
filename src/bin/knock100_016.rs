// 整数値を入力させ、入力値が0でなければ再度入力させ、0であれば終了するプログラム

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  loop {
    let input = get_num("input number".to_string());
    if input == 0 {
        break;
    }
  }
}
