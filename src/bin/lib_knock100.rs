/* 100本ノック用によく使用する処理をモジュール化 */

// 標準ライブラリのioモジュールのショートカット
// print!マクロ（get_numで使用）を使うために必要
use std::io::{Write, self};

// テスト用Hello World
pub fn test() {
  println!("Hello World!");
}

// 引数をそのまま出力するテスト
pub fn echo_test(str: String) {
  println!("{}", str);
}

// 数値の入力を促し、その数値を返す
pub fn get_num(label: String) -> i32 {
  print!("{}: ", label);
  let mut input = String::new();
  io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut input).ok();
  input.trim().parse().unwrap() // 数値に変換して返す(Rustでは最後の式が返り値になる)
}
