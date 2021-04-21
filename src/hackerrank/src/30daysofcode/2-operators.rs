use std::io;

fn main() {
  let stdin = io::stdin();

  let mut meal_cost = String::new();
  stdin.read_line(&mut meal_cost).expect("failed to read stdin");
  let meal_cost: f32 = meal_cost.trim().parse().expect("invalid input");

  let mut tip_percent = String::new();
  stdin.read_line(&mut tip_percent).expect("failed to read stdin");
  let tip_percent: f32 = tip_percent.trim().parse().expect("invalid input");

  let mut tax_percent = String::new();
  stdin.read_line(&mut tax_percent).expect("failed to read stdin");
  let tax_percent: f32 = tax_percent.trim().parse().expect("invalid input");

  let tip: f32 = meal_cost * tip_percent / 100.0;
  let tax: f32 = meal_cost * tax_percent / 100.0;

  println!("The total meal cost is {:.0} dollars.", tip + meal_cost + tax);
}
