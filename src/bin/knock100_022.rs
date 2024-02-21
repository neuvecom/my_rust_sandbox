/*
整数値を入力させ、その値が-10以下、または、10以上であれば
OKと表示するプログラムを作成せよ。
*/

fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  if input <= -10 || input >= 10 {
    println!("OK");
  }
}
