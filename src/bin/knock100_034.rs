// 整数値を入力させ、1から9まで、
// 入力値と入力値+1以外を表示するプログラムを作成せよ。
// 入力値が9の場合は9のみ表示しない。
fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  for i in 1..10 {
    if i != input && i != input + 1 {
      print!("{} ", i);
    }
  }
  println!();
}
