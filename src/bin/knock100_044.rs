/*
換算したい金額（円単位）と1ドル何円かを整数値で入力すると、
入力した金額が何ドル何セントか計算して表示するプログラム
1セントは1/100ドルである。結果は整数値でよい（1セント未満の端数切り捨て）。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut input = Vec::new();
    input.push(get_num("Hou much Yen?".to_string()));
    input.push(get_num("How much rate Dollar?".to_string()));
    println!("{:?}", input);
    let dollar = input[0] / input[1];
    let cent = input[0] % input[1] * 100 / input[1];
    println!("{} dollar {} cent", dollar, cent);
}
