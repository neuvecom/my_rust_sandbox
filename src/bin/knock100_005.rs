/*
整数値を2つ入力させ、それらの値の和、差、積、商と余りを求めるプログラムを作成せよ。
なお、差と商は1つ目の値から2つ目の値を引いた、あるいは割った結果とする。
余りは無い場合も0と表示するのでよい。
*/

use std::io::{Write, self};

fn main() {
    // 初期化
    let mut input_1st = String::new();
    let mut input_2nd = String::new();
    // 1番目の数値を入力
    print!("input 1st number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_1st).ok();
    let input_1st = input_1st.trim();
    let output_1st: i32 = input_1st.parse().unwrap();
    // 2番目の数値を入力
    print!("input 2nd number: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_2nd).ok();
    let input_2nd = input_2nd.trim();
    let output_2nd: i32 = input_2nd.parse().unwrap();
    // 結果を出力
    println!("Sum is: {}", output_1st + output_2nd);
    println!("Difference is: {}", output_1st - output_2nd);
    println!("Product is: {}", output_1st * output_2nd);
    println!("Quotient is: {}, Remainder is: {}", output_1st / output_2nd, output_1st % output_2nd);
}
