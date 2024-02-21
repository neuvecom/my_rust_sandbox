/*
整数値を入力させ、
その値が一桁の自然数かそうでないか判定するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let input = get_num("input number".to_string());
    if input < 10 && input > 0 {
        println!("one digit natural number");
    } else {
        println!("not one digit natural number");
    }
}
