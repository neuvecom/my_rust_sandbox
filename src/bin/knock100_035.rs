/*
{3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
整数値を入力させ、要素番号が入力値である配列要素の値を表示するプログラムを作成せよ。
入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
*/

fn main() {
    let a = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    println!("Input number");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let i: usize = s.trim().parse().unwrap();
    println!("{}", a[i]);
}
