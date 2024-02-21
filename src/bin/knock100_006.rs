/*
整数値を入力させ、値が0ならzeroと表示するプログラム。
整数値を入力させ、値が0ならzero、0でなければnot zeroと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let output = get_num("input number".to_string());
    if output == 0 {
        println!("The input number is zero.");
    } else {
        println!("{} is not zero.", output);
    }
}
