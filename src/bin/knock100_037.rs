/*
{3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
整数値を入力させ、要素番号が入力値の配列要素の値を参照し、
さらにその参照した値を要素番号とする配列要素の値を参照して表示するプログラム
入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    // 配列を宣言
    let a = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let input:usize = get_num("input number".to_string()).try_into().unwrap();
    let index = a[input];
    println!("{:?}", a);
    println!("{}", a[index]);
}
