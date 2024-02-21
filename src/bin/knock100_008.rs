/*
整数値を入力させ、値が正であればpositiveと表示するプログラム。ただし0は正には含まない
整数値を入力させ、値が正であればpositive、負であればnegative、0であればzeroと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let output = get_num("input number".to_string());
    if output > 0 {
        println!("{} is positive", output);
    } else if output < 0 {
        println!("{} is negative.", output);
    } else {
        println!("The input number is zero.");
    }
}
