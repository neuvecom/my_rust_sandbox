/*
西暦を入力したらその年が閏（うるう）年かどうかを判定するプログラム
なお、4で割り切れる年のうち、100で割り切れないか、400で割り切れる年は閏年である。
*/

mod lib_knock100_get_num;
use crate::lib_knock100_get_num::get_num;

fn main() {
    let year: i32 = get_num("西暦を入力".to_string());
    let is_leap_year: bool = if year % 4 == 0 {
        if year % 100 != 0 || year % 400 == 0 {
            true
        } else {
            false
        }
    } else {
        false
    };
    println!("{}年は閏年{}", year, if is_leap_year { "です" } else { "ではありません" });
}
