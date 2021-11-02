/* This code taken from
   http://en.wikipedia.org/wiki/Rust_(programming_language) */
/* This function creates ten "tasks" that all execute concurrently.
   To verify this, run the program several times and observe the irregular
   order in which each task's output is printed. */
fn main() {
    // This string is immutable, so it can safely be accessed from multiple tasks.
    let greeting = "Hello";
    // `for` loops work with any type that implements the `Iterator` trait.
    for num in range(0, 10) {
        spawn(proc() {
            // `println!` is a macro that statically typechecks a format string.
            // Macros are structural (as in Scheme) rather than textual (as in C).
            println!("{:s} from lightweight thread number {:i}", greeting, num);
        });
    }
}

