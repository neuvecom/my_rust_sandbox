// 整数値を入力させ、その値が-10以上0未満、または、10以上であればOK、
// そうでなければNGと表示するプログラムを作成せよ。
fn main() {
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input >= 10 || (input > -10 && input <= 0){
        println!("OK");
    } else {
        println!("NG");
    }
}
