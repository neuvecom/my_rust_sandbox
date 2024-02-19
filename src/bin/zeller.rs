fn main() {
  let year: i32 = 2023;
  let month: i32 = 8;
  let day: i32 = 4;

  let zeller_iso: i32 = zeller(year, month, day);
  println!("{:04}年{:02}月{:02}日({})", year, month, day, zeller_iso);
}

fn zeller(y: i32, m: i32, d: i32) -> i32 {
  let century: i32 = y / 100;
  let gamma: i32 = if (4..1582).contains(&y) {
    6 * century + 5 // ユリウス暦
  } else {
    5 * century + century / 4 // グレゴリオ暦
  };

  // 1月と2月は前年の13月と14月として扱う
  let (year, month) = if m <= 2 {
    (y - 1, m + 12)
  } else {
    (y, m)
  };
  let year_l2: i32 = year % 100; // 西暦の下2桁

  // ツェラーの公式で計算（ISO 8601に準拠）
  (d + (26 * (month + 1)) / 10 + year_l2 + (year_l2 / 4) + gamma + 5) % 7 + 1
}
