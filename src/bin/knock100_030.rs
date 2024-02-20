// 整数値を入力させ、その個数だけ*を表示するプログラムを作成せよ。
// 入力値が0以下の値の場合は何も書かなくてよい。
fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  if input > 0 {
    println!("{}", "*".repeat(input as usize));
  }
}
