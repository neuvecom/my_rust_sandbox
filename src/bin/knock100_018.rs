// 配列を入力値で初期化

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("整数値を入力してください".to_string());
  let array: [i32; 10] = [input; 10];
  println!("{:?}", array);
  // 回答例はFor文をつかって表示されているようだけど、
  // 文章を読むと順に表示とは書いていないので上記とした
  // for i in 0..10 {
  //   println!("{}", array[i]);
  // }
}
