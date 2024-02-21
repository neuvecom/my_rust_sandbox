/*
{3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
整数値を2つ入力させ、要素番号が入力値である2つの配列要素の値の積を計算して表示するプログラムを作成せよ。
入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
*/

fn main() {
    let arr: [i32; 10] = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    println!("Input number");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index1: usize = input.trim().parse().unwrap();
    input.clear();
    println!("Input number");
    std::io::stdin().read_line(&mut input).unwrap();
    let index2: usize = input.trim().parse().unwrap();
    println!("{}", arr[index1] * arr[index2]);
}