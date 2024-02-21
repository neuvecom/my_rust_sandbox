/*
自然数の入力値を素因数分解して結果を表示するプログラムを作成せよ。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let input_num: i32 = get_num("input number".to_string());
    let mut num: i32 = input_num;
    let mut factor: i32 = 2;
    let mut factors: Vec<i32> = Vec::new();
    while num > 1 {
        if num % factor == 0 {
            factors.push(factor);
            num /= factor;
        } else {
            factor += 1;
        }
    }
    println!("{} = {}", input_num, factors.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" × "));
}
