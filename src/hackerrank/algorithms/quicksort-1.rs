use std::io;

fn partition<T: Ord>(v: &mut [T]) {
  let last_index = v.len() - 1;
  v.swap(0, last_index);

  let mut store_index = 0;
  for i in 0..last_index {
    if v[i] < v[last_index] {
      v.swap(i, store_index);
      store_index += 1;
    }
  }

  v.swap(store_index, v.len()-1);
}

fn main() {
  let stdin = io::stdin();

  let mut len = String::new();
  stdin.read_line(&mut len).expect("failed to read stdin");
  // let len: u16 = len.trim().parse().expect("invalid input");

  let mut arr = String::new();
  stdin.read_line(&mut arr).expect("failed to read stdin");
  let mut arr: Vec<i16> = arr.split(' ').map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| s.parse().unwrap())
    .collect();

  partition(&mut arr);

  println!("{}", arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
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
