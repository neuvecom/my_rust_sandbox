/*
異なる整数値を2つ入力し、それぞれ別の変数に格納する。
そして、それらの変数の値を入れ替えた後、それぞれの変数の値を表示するプログラム
単に順序を変えて表示するだけではダメ。必ず変数の値を入れ替えること。
下の実行例の場合、まず変数aに2、bに5が入力された状態から、
aの値が5、bの値が2になるように入れ替える。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut a = get_num("a".to_string());
    let mut b = get_num("b".to_string());
    let tmp: i32 = a;
    a = b;
    b = tmp;
    println!("a = {}, b = {}", a, b);
}
