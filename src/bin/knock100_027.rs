/*
整数値を入力させ、1からその値までの総和を計算して表示するプログラムを作成せよ。
ただし、0以下の値を入力した場合は0と表示する。
*/

fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  let sum = (1..=input).sum::<i32>();
  println!("{}", if input <= 0 { 0 } else { sum });
}
