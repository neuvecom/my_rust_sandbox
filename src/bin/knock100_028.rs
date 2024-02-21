/*
整数値を入力させ、その値の階乗を表示するプログラムを作成せよ。
ただし、0以下の値を入力した場合は1と表示する。
*/

fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  let factorial = (1..=input).product::<i32>();
  println!("{}", if input <= 0 { 1 } else { factorial });
}
