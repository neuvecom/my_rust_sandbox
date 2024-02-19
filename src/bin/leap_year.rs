fn main() {
  let years_to_check = [1968, 2000, 2004, 2100, 2400];
  for year in years_to_check {
    if is_leap_year(year) {
      println!("{:04} は閏年", year);
    } else {
      println!("{:04} は閏年でない", year);
    }
  }
}

fn is_leap_year(year: i32) -> bool {
  match (year % 4, year % 100, year % 400) {
    (_, _, 0) => true, // 400 で割り切れる場合は閏年
    (_, 0, _) => false, // 上記を満たさず、100 で割り切れる場合は閏年でない
    (0, _, _) => true, // 上記を満たさず、4 で割り切れる場合は閏年
    _ => false, // それ以外は閏年でない
  }
}