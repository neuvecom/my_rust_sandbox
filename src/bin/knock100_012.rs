// 整数値を入力させ、その値の回数だけHello World!を繰り返して表示するプログラムを作成せよ。
fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: u32 = input.trim().parse().unwrap();
  for _ in 0..input {
    println!("Hello World!");
  }
}
