/*
0以上の整数値を5つ入力させ、
それぞれの値に対して、次の形式でグラフを描くプログラム
形式：左端に値を表示し、適切に空白を空けて":"を書く（:で揃えるためにタブ\tを使う）
その後ろに値の数だけ*を描くが、5個おきに空白を１つ入れる。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut data: Vec<i32> = vec![];
    for _ in 0..5 {
        let input = get_num("input number(>0)".to_string());
        data.push(input);
    }
    println!("{:?}", data);
    for i in 0..5 {
        print!("{}\t:", data[i]);
        for j in 1..data[i]+1 {
            if j % 5 == 0 {
                print!("* ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}
