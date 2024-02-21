/*
まずデータの個数を入力させ、次にデータの個数だけ整数値を入力させる。
この入力データの中で最大値と最小値を求め表示するプログラム
データの個数は100個までとする。
なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
入力のためのメッセージは不要である（実行例を参照すること）。
*/
/* TODO:
[ ] run.sh ファイル指定に対応させる
[ ] run.shの中で、cargo run にファイスの情報を引数を渡す
*/

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // cargo run --bin knock100_054 tear ytre yuih
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", args[1]);

    // let mut max: i32 = std::i32::MIN;
    // let mut min: i32 = std::i32::MAX;
    // let mut input: String = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let n: i32 = input.trim().parse().unwrap();
    // for _ in 0..n {
    //     input.clear();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let num: i32 = input.trim().parse().unwrap();
    //     if num > max {
    //         max = num;
    //     }
    //     if num < min {
    //         min = num;
    //     }
    // }
    // print!("max: {}, ", max);
    // println!("min: {}", min);
}
