// 列挙型で曜日データの構成を行う
enum DayOfWeek {
  Sunday,
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
}

// 数値から列挙型への変換を行う
impl DayOfWeek {
  fn from_iso_number(iso_number: i32) -> DayOfWeek {
    match iso_number {
      1 => DayOfWeek::Monday,
      2 => DayOfWeek::Tuesday,
      3 => DayOfWeek::Wednesday,
      4 => DayOfWeek::Thursday,
      5 => DayOfWeek::Friday,
      6 => DayOfWeek::Saturday,
      7 => DayOfWeek::Sunday,
      _ => unreachable!(),
    }
  }

  // 列挙型から曜日の文字列を取得
  fn to_ja(&self) -> char {
    match self {
      DayOfWeek::Sunday => '日',
      DayOfWeek::Monday => '月',
      DayOfWeek::Tuesday => '火',
      DayOfWeek::Wednesday => '水',
      DayOfWeek::Thursday => '木',
      DayOfWeek::Friday => '金',
      DayOfWeek::Saturday => '土',
    }
  }
}

fn main() {
  let year: i32 = 2023;
  let month: i32 = 8;
  let day: i32 = 4;

  let zeller_iso: i32 = zeller(year, month, day);
  let day_of_week: DayOfWeek = DayOfWeek::from_iso_number(zeller_iso);
  let day_of_week_ja: char = day_of_week.to_ja();
  println!("{:04}年{:02}月{:02}日({})", year, month, day, day_of_week_ja);
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
