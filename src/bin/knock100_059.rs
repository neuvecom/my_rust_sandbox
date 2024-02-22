/*
2つの3x3行列の和を求めて表示するプログラム
行列の値は2次元配列で表現し、繰り返しを使って計算する
3x3行列とは縦3つ、横3つの9つの要素(値)をひとまとめにして扱う
2つの3x3行列の和は、それぞれ同じ位置にある値を足したものとして計算
例えばa12という要素は、1行目2列目の要素という意味
それぞれ同じ位置にある要素を足せばよい。
なお、入力値は1行ずつ3つの値をスペースで区切って入力するようにする
このためには、scanf("%d %d %d", &a[0][0], &a[0][1], &a[0][2]);のように書く(No. 57参照)。
表示は各列の値を揃えるためにタブ(\t)を使う
※入力めんどいので、ランダムで生成する
cargo add rand
*/

use rand::Rng;
use std::num::Wrapping;

fn main() {
  let queue1 = make_queue(3, 3);
  let queue2 = make_queue(3, 3);
  println!("行列1: {:?}",queue1 );
  println!("行列2: {:?}",queue2 );
  println!("行列1と行列2の和:");
  for _i in 0..3 {
    for _j in 0..3 {
      // Rustでの整数オーバーフローまとめ
      // https://qiita.com/garkimasera/items/c5e06de1a7c66aa7652a
      print!("{}\t", Wrapping(queue1[_i][_j]) + Wrapping(queue2[_i][_j]));
    }
    println!();
  }
}

// ランダムでnxn行列を生成する
fn make_queue(row:i8, col: i8) -> Vec<Vec<i8>> {
  let mut rng = rand::thread_rng();
  let mut queue: Vec<Vec<i8>> = vec![];
  for _ in 0..row {
    let mut row: Vec<i8> = vec![];
    for _ in 0..col {
      let i: i8 = rng.gen();
      row.push(i);
    }
    queue.push(row);
  }
  queue
}
