fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}

fn main() { bool_to_word(true); }

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
}
