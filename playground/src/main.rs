fn main() {
  let c: char = 'c';
  println!("{}", c.to_uppercase());

  let str = "chapman".to_string();
  let iter = str.chars().into_iter();
  str.next();
  println!("{}", str);
}
