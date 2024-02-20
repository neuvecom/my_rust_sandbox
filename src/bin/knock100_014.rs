// 整数値を入力させ、入力値から0まで数を1ずつ減らして表示するプログラムを作成せよ。
fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: u32 = input.trim().parse().unwrap();
  for i in (0..=input).rev() {
    println!("{}", i);
  }
}
