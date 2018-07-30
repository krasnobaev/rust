use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut arr_size = String::new();
  stdin.read_line(&mut arr_size).expect("failed to read stdin");
  let arr_size: usize = arr_size.trim().parse().expect("invalid input");

  let mut arr: Vec<u32> =
    stdin.lock()
      .lines().next().unwrap().unwrap()
      .split(' ').map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect();

  let mut n_of_total_swaps = String::new();
  stdin.read_line(&mut n_of_total_swaps).expect("failed to read stdin");
  let mut n_of_total_swaps: u32 = 0;

  for _i in 0..arr_size {
    let mut n_of_swaps: u32 = 0;

    for j in 0..arr_size-1 {
      if arr[j] > arr[j + 1] {
        let t:u32 = arr[j + 1];
        arr[j + 1] = arr[j];
        arr[j] = t;

        n_of_swaps += 1;
        n_of_total_swaps += 1;
      }
    }

    if n_of_swaps == 0 {
      break;
    }
  }

  println!("Array is sorted in {} swaps.", n_of_total_swaps);
  println!("First Element: {}", arr[0]);
  println!("Last Element: {}", arr[arr_size-1]);
}
