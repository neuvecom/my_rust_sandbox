use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
  let mut rng: ThreadRng = rand::thread_rng();
  let arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();

  println!("{:?}", arr);

  // bubble_sort(&mut arr);
}