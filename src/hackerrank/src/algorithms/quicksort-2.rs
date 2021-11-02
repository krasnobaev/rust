use std::io;

fn quick_sort<T: Ord + std::fmt::Display>(v: &mut [T]) {
  let len = v.len();
  if len >= 2 {
    let pivot_index = partition(v);
    quick_sort(&mut v[0..pivot_index]);
    quick_sort(&mut v[pivot_index+1..len]);
  }

  if len > 1 {
    println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
  }
}

fn partition<T: Ord>(v: &mut [T]) -> usize {
  let len = v.len();
  let pivot_index = 0;
  let last_index = len - 1;
  v.swap(pivot_index, last_index);

  let mut store_index = 0;
  for i in 0..last_index {
    if v[i] < v[last_index] {
      v.swap(i, store_index);
      store_index += 1;
    }
  }

  v.swap(store_index, len-1);
  store_index
}

fn main() {
  let stdin = io::stdin();

  let mut len = String::new();
  stdin.read_line(&mut len).expect("failed to read stdin");

  let mut arr = String::new();
  stdin.read_line(&mut arr).expect("failed to read stdin");
  let mut arr: Vec<i16> = arr.split(' ').map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| s.parse().unwrap())
    .collect();

  quick_sort(&mut arr);
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn test1() {
    let mut arr = [4,5,3,7,2];
    partition(&mut arr);
    assert_eq!(arr, [3,2,4,5,7]);
  }

}
