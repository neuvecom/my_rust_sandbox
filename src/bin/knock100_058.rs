/*
0以上の整数値を5つ入力させ、
それぞれの値に対して、次の形式でグラフを描くプログラム
形式：左端に値を表示し、適切に空白を空けて":"を書く（:で揃えるためにタブ\tを使う）
その後ろに値の数だけ*を描くが、5個おきに空白を１つ入れる。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    // let input: Vec<i32> = vec![3, 7, 1, 5, 4];
    let mut sum = 0;
    // let input = get_num("input number".to_string());
    for _ in 0..5 {
      let input = get_num("input number".to_string());
      sum += input;
    }
    for i in sun {
        print!("{}\t:", i);
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
