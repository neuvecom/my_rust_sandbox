/*
まず受験者数を入力させ、
次に受験者数ごとに英語、数学、国語の点数をスペースで区切って入力させる
（具体的な入力方法は下記のscanfの使い方の説明、および入力データの中身を見よ）。
入力が終了したら、英語、数学、国語の平均点、および各受験生の合計点を計算して表示するプログラム
受験者数は100人までとする。
なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
入力のためのメッセージは不要である（実行例を参照すること）。
C++向けヒント: scanf("%d %d %d", &eng, &math, &lang);
入力データファイルの構造
1行目: 受験者数n
2行目からn行: 英語、数学、国語の点数
*/

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // コマンドライン引数からファイル名を取得
    let args: Vec<String> = env::args().collect();
    // ファイルを開く
    let filepath = "./057data/".to_string() + &args[1];
    let file = File::open(filepath).unwrap();
    // 合計点を初期化
    let mut english_total = 0;
    let mut math_total = 0;
    let mut japanese_total = 0;
    // ファイルの中身を読む
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    // 受験者数を取得
    let num_students = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut student_totals: Vec<usize> = Vec::with_capacity(num_students.try_into().unwrap());
    println!("サンプル数:{}", num_students);
    // 合計点を計算
    for line in lines {
      let input: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
      english_total += input[0];
      math_total += input[1];
      japanese_total += input[2];
      student_totals.push((input[0] + input[1] + input[2]).try_into().unwrap());
    }
    // 平均点を計算
    let english_average = english_total / num_students;
    let math_average = math_total / num_students;
    let japanese_average = japanese_total / num_students;
    print!("平均点 英語:{}, ", english_average);
    print!("数学:{}, ", math_average);
    println!("国語:{}", japanese_average);
    // 合計点を表示
    println!("個人合計点");
    for (i, total) in student_totals.iter().enumerate() {
        println!("{}: {}", i, total);
    }
}
