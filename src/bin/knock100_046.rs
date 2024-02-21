/*
神山美術館の入場料金は、一人600円であるが、5人以上のグループなら一人550円、
20人以上の団体なら一人500円である。
人数を入力し、入場料の合計を計算するプログラムを作成せよ。
*/

fn main() {
    let individual_fare: i32 = 600;
    let group_fare: i32 = 550;
    let organization_fare: i32 = 500;
    let input_people: i32 = 20;
    let fare: i32 = if input_people < 5 {
        individual_fare * input_people
    } else if input_people < 20 {
        group_fare * input_people
    } else {
        organization_fare * input_people
    };
    println!("料金: {}円", fare);
}
