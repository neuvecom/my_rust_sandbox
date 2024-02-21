/*
整数値を入力させ、その値が-10未満ならrange 1、
-10以上0未満であればrange 2、
0以上であればrange 3、
と表示するプログラム
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let input = get_num("input number".to_string());
    if input < -10 {
        println!("range 1");
    } else if input < 0 && input >= -10{
        println!("range 2");
    } else {
        println!("range 3");
    }
}
