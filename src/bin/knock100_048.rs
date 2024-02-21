/*
最初に2以上の整数値を入力し、次の規則で計算し、計算回数と計算結果を表示し、
計算結果が1になるまで繰り返すプログラム
規則：ある値が偶数ならその値を1/2にする。奇数ならその値を3倍して1を足す。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut input = get_num("input number(>2)".to_string());
    let mut count: i32 = 0;
    while input != 1 {
        if input % 2 == 0 {
            input /= 2;
        } else {
            input = input * 3 + 1;
        }
        count += 1;
        println!("{}: {}", count, input);
    }
}
