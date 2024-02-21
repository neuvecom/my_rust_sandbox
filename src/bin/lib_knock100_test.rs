/* 100本ノック用によく使用する処理をモジュール化(test) */

// テスト用Hello World
pub fn test() {
  println!("Hello World!");
}

// 引数をそのまま出力するテスト
pub fn echo_test(str: String) {
  println!("{}", str);
}
