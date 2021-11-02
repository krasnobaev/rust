// use std::io;
extern crate getopts;
use getopts::Options;

#[macro_use]
extern crate lazy_static;

// mod naive;
// mod struct;
mod recursive_unsafe;
mod recursive_mutex;
mod recursive_mutex_lazy_static;
mod recursive_mutex_once_cell;
mod best_exe;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} ALGORITHM N", program);
    print!("{}", opts.usage(&brief));
}

fn print_result(n: u32, result: u32) {
    println!("RESULT: {}th prime is {}", n, result);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.reqopt("a", "alg", "algorithm", "ALGORITHM");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f) }
    };
    let algorithm = matches.opt_str("a");

    let n = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    // let stdin = io::stdin();
    // let mut str = String::new();
    // stdin.read_line(&mut str).expect("failed to read stdin");
    // let n: u32 = str.trim().parse().expect("invalid input");

    match (algorithm, n) {
        (Some(s), n) => match (s.trim().parse(), n.trim().parse()) {
            (Ok(1), Ok(n)) => print_result(n, recursive_unsafe::nth(n-1)),
            (Ok(2), Ok(n)) => print_result(n, recursive_mutex::nth(n-1)),
            (Ok(3), Ok(n)) => print_result(n, recursive_mutex_lazy_static::nth(n-1)),
            (Ok(4), Ok(n)) => print_result(n, recursive_mutex_once_cell::nth(n-1)),
            (Ok(5), Ok(n)) => print_result(n, recursive_unsafe::nth(n-1)),
            (Ok(6), Ok(n)) => print_result(n, best_exe::nth(n-1)),
            _ => print_usage(&program, opts), // TODO print algo list
        },
        // for i in 1..=10 {
        //     println!("RESULT: {}th prime is {}", i, r#unsafe::nth(i-1));
        // }
        _ => print_usage(&program, opts),
    };
}
