use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pairs(k: i32, arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut a = arr.to_vec();
    let l = a.len();
    a.sort();

    for i in 0..l {
        for j in 1..l {
            if a[j] - a[i] == k {
                sum += 1;
            }
        }
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pairs(k, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
