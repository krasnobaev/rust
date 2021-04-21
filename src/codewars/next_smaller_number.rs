fn next_smaller_number(n: u64) -> Option<u64> {
  let mut digs = n.to_string().chars().collect::<Vec<char>>();
  for i in (0..digs.len()).rev() {
    for j in (0..i).rev() {
      if (i == 0 || j == 0) && (digs[i] == '0' || digs[j] == '0') { continue }
      if digs[i] < digs[j] {
        digs.swap(i, j);
        let num: String = digs.into_iter().collect();
        return Some(num.parse::<u64>().unwrap());
      }
    }
  }

  None
}
