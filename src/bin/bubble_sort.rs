use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
  // 初期化
  let mut rng: ThreadRng = rand::thread_rng();
  let mut arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
  // ソート前の配列を表示
  println!("created array\n{:?}", arr);
  // ソート
  let (cmp, swap) = bubble_sort(&mut arr);
  // ソート後の配列を表示
  println!("bubble sort\n{:?}", arr);
  println!("check: {}, swap: {}", cmp, swap);
}

// バブルソート
fn bubble_sort(a: &mut Vec<i32>) -> (i32, i32) {
  // 比較回数と交換回数
  let mut cmp_count: i32 = 0;
  let mut swap_count: i32 = 0;
  // 配列の長さ
  let n: usize = a.len();
  // ソート
  for i in 0..n-1 {
    for j in 0..n-i-1{
      if a[j] > a[j+1] {
        a.swap(j, j+1);
        swap_count += 1;
      }
      cmp_count += 1;
    }
  }
  (cmp_count, swap_count)
}