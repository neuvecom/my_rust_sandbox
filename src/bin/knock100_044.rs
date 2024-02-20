// 換算したい金額（円単位）と1ドル何円かを整数値で入力すると、
// 入力した金額が何ドル何セントか計算して表示するプログラムを作成せよ。
// 1セントは1/100ドルである。結果は整数値でよい（1セント未満の端数切り捨て）。
fn main() {
    println!("Input yen and dollar");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("yen: {}", input[0]);
    println!("dollar: {}", input[1]);
    let dollar = input[0] / input[1];
    let cent = input[0] % input[1] * 100 / input[1];
    println!("{} dollar {} cent", dollar, cent);
}
