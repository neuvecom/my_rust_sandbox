// 整数値を3つ入力させ、
// それらの値が小さい順（等しくてもよい）に並んでいるか判定し、
// 小さい順に並んでいる場合はOK、そうなっていない場合はNG
// と表示するプログラムを作成せよ。
fn main() {
    println!("Input 3 numbers");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    if input[0] <= input[1] && input[1] <= input[2] {
        println!("OK");
    } else {
        println!("NG");
    }
}
