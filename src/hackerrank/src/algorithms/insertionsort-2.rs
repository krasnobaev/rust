use std::io;

fn insertion_sort<T: Ord+Clone>(v: &[T]) -> Vec<Vec<T>> {


  vec![vec![v[0].clone()]]
}

fn main() {
  let stdin = io::stdin();

  let mut len = String::new();
  stdin.read_line(&mut len).expect("failed to read stdin");
  // let len: u16 = len.trim().parse().expect("invalid input");

  let mut arr = String::new();
  stdin.read_line(&mut arr).expect("failed to read stdin");
  let arr: Vec<i16> = arr.split(' ').map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| s.parse().unwrap())
    .collect();

  let result = insertion_sort(&arr);
  for arr in result {
    println!("{}", arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = [1,4,3,5,6,2];
    let ra = [
      [1,4,3,5,6,2],
      [1,3,4,5,6,2],
      [1,3,4,5,6,2],
      [1,3,4,5,6,2],
      [1,2,3,4,5,6],
    ];

    let res = insertion_sort(&input);
    assert!(ra.len() == res.len(), "result differ in length of iterations");

    assert!(ra[0].len() == res[0].len());
    assert!(ra[0].iter().zip(res[0].iter()).all(|(a, b)| a == b), "Arrays are not equal");
    assert!(ra[1].len() == res[1].len());
    assert!(ra[1].iter().zip(res[1].iter()).all(|(a, b)| a == b), "Arrays are not equal");
    assert!(ra[2].len() == res[2].len());
    assert!(ra[2].iter().zip(res[2].iter()).all(|(a, b)| a == b), "Arrays are not equal");
    assert!(ra[3].len() == res[3].len());
    assert!(ra[3].iter().zip(res[3].iter()).all(|(a, b)| a == b), "Arrays are not equal");
    assert!(ra[4].len() == res[4].len());
    assert!(ra[4].iter().zip(res[4].iter()).all(|(a, b)| a == b), "Arrays are not equal");
  }

}
