/*
整数値を入力させ、
その値が一桁の自然数かそうでないか判定するプログラムを作成せよ。
*/

fn main() {
    println!("Input number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input < 10 && input > 0 {
        println!("one digit natural number");
    } else {
        println!("not one digit natural number");
    }
}
