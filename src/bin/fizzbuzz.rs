
fn main() {
  let n: usize = 20;
  println!("FizzBuzz");
  fizz_buzz(n);
}

fn fizz_buzz(n: usize) {
  for i in 1..=n {
    if i % 15 == 0 {
      println!("{} : FizzBuzz", i);
    } else if i % 3 == 0 {
      println!("{} : Fizz", i);
    } else if i % 5 == 0 {
      println!("{} : Buzz", i);
    } else {
      println!("{}", i);
    }
  }
}

