// 整数値を2つ入力させ、1つめの値を2つめの値で割った結果を表示し、
// 続けてその結果に2つめの値を掛けた結果を表示するプログラムを作成せよ。
// 計算はすべて整数型で行うこと
//（割り切れない場合は自動的に小数点以下が切り捨てられる）。
fn main() {
  println!("整数値をスペースで区切って2つ入力してください。");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
  let result = input[0] / input[1];
  println!("{}", result);
  let result = result * input[1];
  println!("{}", result);
}
