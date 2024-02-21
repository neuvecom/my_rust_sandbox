/*
1から100までの値を繰り返しで表示するが、
3の倍数の時はfoo、5の倍数の時はbarと数字の代わりに表示するプログラム
なお、3と5の両方の倍数の時はfoobarと表示される。
*/

fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}: foobar", i);
        } else if i % 3 == 0 {
            println!("{}: foo", i);
        } else if i % 5 == 0 {
            println!("{}: bar", i);
        } else {
            println!("{}: {}", i, i);
        }
    }
}
