/*
指定した金額を100円玉と10円玉と1円玉だけで、
できるだけ少ない枚数で支払いたい。
金額を入力するとそれぞれの枚数を計算して表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut amount: i32 = get_num("金額を入力してください".to_string());
    let num_100: i32 = amount / 100;
    amount = amount % 100;
    let num_10: i32 = amount / 10;
    amount = amount % 10;
    let num_1: i32 = amount;

    print!("100円玉: {}枚, ", num_100);
    print!("10円玉: {}枚, ", num_10);
    println!("1円玉: {}枚", num_1);
}
