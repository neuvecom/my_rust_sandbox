// 整数値を5回入力させ、それらの値の合計を表示するプログラムを繰り返しを使って作成せよ。

fn main() {
  let mut sum = 0;
  for _ in 0..5 {
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    sum += input;
  }
  println!("{}", sum);
}
