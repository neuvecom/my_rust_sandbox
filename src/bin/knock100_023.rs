// 整数値を入力させ、その値が-5以上10未満であればOK、
// そうでなければNGと表示するプログラムを作成せよ。
fn main() {
    println!("整数値を入力してください");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input >= -5 && input < 10 {
        println!("OK");
    } else {
        println!("NG");
    }
}
