/* This code taken from
   http://en.wikipedia.org/wiki/Rust_(programming_language) */
fn main() {
    let array = ["Arrays", "are", "for", "values", "of", "the", "same", "type"];
    let tuple = ("Tuples", 'r', 4i, 0xDEADBEEFi);

    // `match` expressions are the typical way of employing pattern-matching,
    // and are somewhat analogous to the `switch` statement from C and C++.
    let uno = match array {
        // Below is an array pattern, which mirrors the syntax for array literals.
        // An underscore in a pattern will ignore a single element.
        // A double-dot `..` in a pattern will ignore multiple elements.
        [_, _, _, values, ..] => values
    };

    // Pattern-matching can also be employed when declaring variables.
    // This will declare two new variables in the current scope, `dos` and `tres`.
    let (_, dos, _, tres) = tuple;

    println!("{:s} {:c} {:x}!", uno, dos, tres);  // Prints "values r deadbeef!"
}

