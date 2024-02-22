/*
まずデータの個数を入力させ、次にデータの個数だけ整数値を入力させる。
この入力データの中で最大値と最小値を求め表示するプログラム
データの個数は100個までとする。
なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
入力のためのメッセージは不要である（実行例を参照すること）。
*/
/* TODO:
[x] run.shをファイル指定に対応させる
[x] run.shの中で、cargo run にファイスの情報を引数を渡す
*/
/* MEMO:
cargo run --bin knock100_054 tear ytre yuih
["/Users/yoshiharusato/work_tmp/Rust/my_rust_sandbox/target/debug/knock100_054", "tear", "ytre", "yuih"]
*/

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = "./054data/".to_string() + &args[1];
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut max: i32 = std::i32::MIN;
    let mut min: i32 = std::i32::MAX;
    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        if num > max { max = num; }
        if num < min { min = num; }
    }
    print!("max: {}, ", max);
    println!("min: {}", min);
}
