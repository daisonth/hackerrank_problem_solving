use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn morganAndString(a: &str, b: &str) -> String {
    let mut x = a.to_string() + "z";
    let mut y = b.to_string() + "z";
    let mut r = String::new();

    while !r.ends_with('z') {
        if x < y {
            r.push(x.chars().next().unwrap());
            x = (&x[1..].to_string()).clone();
        } else {
            r.push(y.chars().next().unwrap());
            y = (&y[1..].to_string()).clone();
        }
    }
    r.pop();
    r
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let a = stdin_iterator.next().unwrap().unwrap();

        let b = stdin_iterator.next().unwrap().unwrap();

        let result = morganAndString(&a, &b);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
