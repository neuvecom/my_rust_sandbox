/*
整数値を3つ入力させ、
それらの値が小さい順（等しくてもよい）に並んでいるか判定し、
小さい順に並んでいる場合はOK、そうなっていない場合はNG
と表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut input = Vec::new();
    input.push(get_num("input number 1".to_string()));
    input.push(get_num("input number 2".to_string()));
    input.push(get_num("input number 3".to_string()));
    println!("{:?}", input);
    if input[0] <= input[1] && input[1] <= input[2] {
        println!("OK");
    } else {
        println!("NG");
    }
}
