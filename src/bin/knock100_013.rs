// 整数値を入力させ、0から入力値まで数を1ずつ増やして表示するプログラムを作成せよ。
fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: u32 = input.trim().parse().unwrap();
  for i in 0..=input {
    println!("{}", i);
  }
}
