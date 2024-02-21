/*
初乗り料金が1700mまで610円、それ以降は313mごとに80円のタクシーがある。
このタクシーに乗った距離をm単位で入力し、料金を計算するプログラムを作成せよ。
*/

fn main() {
    let distance: i32 = 1700;
    let base_fare: i32 = 610;
    let additional_fare: i32 = 80;
    let additional_distance: i32 = 313;

    let input_distance: i32 = 2000;

    let fare: i32 = if input_distance <= distance {
        base_fare
    } else {
        base_fare + ((input_distance - distance + additional_distance - 1) / additional_distance) * additional_fare
    };

    println!("料金: {}円", fare);
}
