/*
整数値を入力させ、その値を絶対値にして表示するプログラム
（できれば変数の値を絶対値に変えるようにする）
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let output = get_num("input number".to_string());
    println!("absolute value is {}", output.abs());
}
