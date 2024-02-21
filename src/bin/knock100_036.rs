/*
{3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
整数値を2つ入力させ、要素番号が入力値である2つの配列要素の値の積を計算して表示するプログラム
入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let arr: [i32; 10] = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let index1: usize = get_num("input number".to_string()).try_into().unwrap();
    let index2: usize = get_num("input number".to_string()).try_into().unwrap();
    println!("{:?}", arr);
    println!("{}", arr[index1] * arr[index2]);
}