/*
2次方程式 ax^2 + bx + c = 0 （x^2はxの2乗の意味）の係数a, b, cを入力し、
2次方程式の解が2つの実数解か、重解か、2つの虚数解かを判別するプログラムを作成せよ。
*/

fn main() {
    println!("Input 3 numbers");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("a: {}", input[0]);
    println!("b: {}", input[1]);
    println!("c: {}", input[2]);
    /*
    ax^2 + bx + c = 0  が重解を持つ時の判別式は b^2 - 4ac = 0
    判別式とは、2次方程式 ax^2 + bx + c = 0 の解の性質を表す式
    判別式 > 0 の時、2つの実数解
    判別式 = 0 の時、重解
    判別式 < 0 の時、2つの虚数解
    となる。
    ax^2 + bx + c = 0
    x = (-b ± √(b^2 - 4ac)) / 2a
    */
    let d = input[1] * input[1] - 4 * input[0] * input[2];
    if d > 0 {
        println!("2つの実数解");
    } else if d == 0 {
        println!("重解");
    } else {
        println!("2つの虚数解");
    }
}
