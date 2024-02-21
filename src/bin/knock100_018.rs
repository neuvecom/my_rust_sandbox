// 配列を入力値で初期化

fn main() {
  println!("整数値を入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  let array: [i32; 10] = [input; 10];
  for i in 0..10 {
    println!("{}", array[i]);
  }
}
