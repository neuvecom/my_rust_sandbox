/*
整数値を入力させ、その値が偶数ならばeven、
奇数ならばoddと表示するプログラムを作成せよ。
*/

fn main() {
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
