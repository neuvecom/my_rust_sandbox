/*
2つの3x3行列の和を求めて表示するプログラム
行列の値は2次元配列で表現し、繰り返しを使って計算する
3x3行列とは縦3つ、横3つの9つの要素(値)をひとまとめにして扱う
2つの3x3行列の和は、それぞれ同じ位置にある値を足したものとして計算
例えばa12という要素は、1行目2列目の要素という意味
それぞれ同じ位置にある要素を足せばよい。
なお、入力値は1行ずつ3つの値をスペースで区切って入力するようにする
このためには、scanf("%d %d %d", &a[0][0], &a[0][1], &a[0][2]);のように書く(No. 57参照)。
表示は各列の値を揃えるためにタブ(\t)を使う
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
  let input = get_num("input number1-1".to_string());

}

/* 
1つめの行列
1 2 3
4 5 6
7 8 9
2つめの行列
2 3 4
5 6 7
8 9 1
和
3	5	7	
9	11	13	
15	17	10	

fn main() {
    let matrix1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let matrix2 = [[10, 11, 12], [13, 14, 15], [16, 17, 18]];

    let mut sum_matrix = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            sum_matrix[i][j] = matrix1[i][j] + matrix2[i][j];
        }
    }

    println!("行列の和は:");
    for row in sum_matrix.iter() {
        for element in row.iter() {
            print!("{} ", element);
        }
        println!();
    }
}

2つの3x3行列を定義します。
2つの行列の各要素の合計を計算します。
合計値を新しい行列に格納します。
新しい行列を表示します。


*/