/*
「とんで」を9回
「まわって」を3回繰り返した後
「まわる」と表示して改行する、
を3回繰り返すプログラム
「とんで」「まわって」と3行文の繰り返しは必ず繰り返し構文を使う
*/

fn main() {
    let mut song = sing_a_song("とんで ", 9);
    song += &sing_a_song("まわって ", 3);
    song += "まわ〜る〜〜〜〜♪\n";
    song = sing_a_song(&song , 3);
    println!("{}", song);
}

fn sing_a_song(str: &str, n: i32) -> String {
    let mut lilic = String::new();
    for _ in 0..n {
        lilic += str;
    }
    lilic
}
