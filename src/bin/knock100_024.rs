/*
整数値を入力させ、その値が-10以上0未満、または、10以上であればOK、
そうでなければNGと表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let input = get_num("input number".to_string());
    if input >= 10 || (input > -10 && input <= 0){
        println!("OK");
    } else {
        println!("NG");
    }
}
