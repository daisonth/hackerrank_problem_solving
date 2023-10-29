use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'camelcase' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn camelcase(ss: &str) -> i32 {
    let s: Vec<char> = ss.chars().collect::<Vec<char>>();
    let mut r = 1;
    for i  in 0..s.len() {
        if s[i] < 'a' {
          r+=1;
        }
    }
    r as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = camelcase(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
