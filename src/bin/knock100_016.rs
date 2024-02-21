// 整数値を入力させ、入力値が0でなければ再度入力させ、0であれば終了するプログラムを作成せよ。

fn main() {
  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input == 0 {
        break;
    }
  }
}
