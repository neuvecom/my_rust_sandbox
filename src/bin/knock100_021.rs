/*
整数値を入力させ、その値が5よりも大きく、
かつ、20よりも小さければOKと表示するプログラムを作成せよ。
*/

fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  if input > 5 && input < 20 {
    println!("OK");
  }
}
