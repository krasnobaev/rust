fn elder_age(m: u64, n: u64, l: u64, t: u64) -> u64 {
  // let rowsum = (1 + (m-1 - l)) * (m-1 - l) / 2;
  // println!("rowsum: {}", rowsum);
  // let sum = rowsum * n;
  let mut sum = 0;

  for i in 0..n {
    for j in 0..m {
      if i^j > l {
        sum += ((i^j) - l).max(0)
      }
    }
  }
  sum % t
}
