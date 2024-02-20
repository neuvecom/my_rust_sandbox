// 整数値を入力させ、その値が1ならone、2ならtwo、3ならthree、
// それ以外ならothersと表示するプログラムを
// matchを使って作成せよ。(swicth-case文から変更)
fn main() {
  println!("Input number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: i32 = input.trim().parse().unwrap();
  match input {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("others"),
  }
}
