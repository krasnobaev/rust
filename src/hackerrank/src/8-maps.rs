use std::io;
use std::collections::HashMap;

fn main() {
  let stdin = io::stdin();
  let mut phone_book = HashMap::new();

  let mut num_entries = String::new();
  stdin.read_line(&mut num_entries).expect("failed to read stdin");
  let num_entries: i32 = num_entries.trim().parse().expect("invalid input");
  let mut i: i32 = 0;

  while i < num_entries {
    let mut entry_line = String::new();
    stdin.read_line(&mut entry_line).expect("failed to read stdin");
    let entry_line: String = entry_line.trim().parse().expect("invalid input");
    let entry: Vec<&str> = entry_line.split(' ').collect();

    phone_book.insert(entry[0].to_string(), entry[1].to_string());

    i += 1;
  }

  let mut done = false;

  while !done {
    let mut key = String::new();
    stdin.read_line(&mut key).expect("failed to read stdin");
    let key: String = key.trim().parse().expect("invalid input");
    let keyptr: &str = &*key;

    if key.len() == 0 {
      done = true;
    } else if phone_book.contains_key(keyptr) {
      println!("{}={}", key, phone_book.get(keyptr).unwrap());
    } else {
      println!("Not found");
    }
  }
}
