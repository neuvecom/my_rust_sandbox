// 整数値を入力させ、その個数だけ*を、
// 5個おきに空白（スペース）を入れて表示するプログラムを作成せよ。
// 入力値が0以下の値の場合は何も書かなくてよい。
fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  if input > 0 {
    for i in 1..input {
      if i % 5 == 0 {
        print!("* ");
      } else {
        print!("*");
      }
    }
    println!(" ");
  }
}
