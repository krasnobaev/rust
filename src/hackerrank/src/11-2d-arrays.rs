use std::io::{self, BufRead};

fn glasssum (i: usize, j: usize, arr: &Vec<Vec<i32>>) -> i32 {
  return arr[i-1][j-1] + arr[i-1][j] + arr[i-1][j+1] +
                         arr[i][j] +
         arr[i+1][j-1] + arr[i+1][j] + arr[i+1][j+1];
}

fn main() {
  let stdin = io::stdin();
  let mut arr = vec![vec![0; 6]; 6];

  let mut i = 0;
  while i < 6 {
    arr[i] =
      stdin.lock()
        .lines().next().unwrap().unwrap()
        .split(' ').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    i += 1;
  }

  let mut i:usize = 1;
  let mut max:i32 = i32::min_value();
  let mut cur:i32;
  while i < 5 {
    let mut j:usize = 1;
    while j < 5 {
      cur = glasssum(i, j, &arr);
      if cur > max {
        max = cur;
      }

      j += 1;
    }

    i += 1;
  }

  println!("{}", max);
}
