fn overflow(i: u32) -> u32 {
    println!("depth: {}", i);
    return overflow(i+1)
}

fn main() {
    println!("{}", overflow(0));
}
