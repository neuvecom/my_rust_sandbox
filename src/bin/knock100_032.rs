/*
1から20まで順に表示するが、
5の倍数の場合は数字の代わりにbarと表示するプログラム
*/

fn main() {
  for i in 1..=20 {
    if i % 5 == 0 {
      println!("bar");
    } else {
      println!("{}", i);
    }
  }
}

