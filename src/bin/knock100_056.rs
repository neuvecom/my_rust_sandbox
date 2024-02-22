/*
0〜65535の整数値を入力させ、入力値を16桁の2進数に変換して表示するプログラム
ヒント：
16桁分の2進数を記憶する配列を用意する。
2で割った余り(0か1)を最初の桁の値とし、2で割った値を新たな値とし、
さらに2で割った余りを次の桁の値とする。
これを繰り返していけば1の桁から順に値が求まるので、
表示するときは逆順に表示すればよい。
Gemini:
1. 変換したい10進数をnum変数に代入します。
2. 16桁のbool配列binaryを初期化します。
3. forループを使用して、10進数を2進数に変換します。
4. ループ内で、numを2で割った余りで、現在の桁が1か0かを判断します。
5. 現在の桁が1であれば、binary配列の対応する要素をtrueに設定します。
6. numを2で割って、次の桁に移動します。
7. forループを使用して、binary配列の各要素を順番に表示します。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let mut num = get_num("input number(0-65535)".to_string());
    let mut binary: [bool; 16] = [false; 16]; // 16桁の2進数列
    for i in 0..16 {
        binary[15 - i] = num % 2 == 1;
        num /= 2;
    }
    for bit in binary {
        print!("{}", if bit { 1 } else { 0 });
    }
    println!();
}
