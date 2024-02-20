// 整数値を2つ入力させ、それらの値の和、差、積、商と余りを求めるプログラムを作成せよ。
// なお、差と商は1つ目の値から2つ目の値を引いた、あるいは割った結果とする。
// 余りは無い場合も0と表示するのでよい。
use std::io::{Write, self};

fn main() {
    // 初期化と入力待機
    let num_1st = get_num("1st".to_string());
    let num_2nd = get_num("2nd".to_string());
    // 結果を出力（和・差・積・商・余）
    println!("Sum is: {}", num_1st + num_2nd);
    println!("Difference is: {}", num_1st - num_2nd);
    println!("Product is: {}", num_1st * num_2nd);
    println!("Quotient is: {}", num_1st / num_2nd);
    println!("Remainder is: {}", num_1st % num_2nd);
}

// 入力を待機し、入力された値を数値で返す
fn get_num(label: String) -> i32 {
    print!("input {} number: ", label);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    input.parse().unwrap()
}
