/*
整数値を入力させ、その値が偶数ならばeven、
奇数ならばoddと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let input = get_num("input number".to_string());
    if input % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
