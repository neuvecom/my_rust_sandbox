// 整数値を入力させ、その値が-10未満ならrange 1、
// -10以上0未満であればrange 2、
// 0以上であればrange 3、
// と表示するプログラムを作成せよ。
fn main() {
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    if input < -10 {
        println!("range 1");
    } else if input < 0 && input >= -10{
        println!("range 2");
    } else {
        println!("range 3");
    }
}
