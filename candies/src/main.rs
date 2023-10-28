use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn candies(n: i32, arr: &[i32]) -> i64 {
    let mut can = vec![1; n as usize];
    for i in 1..n as usize {
        if arr[i] > arr[i - 1] {
            can[i] = can[i - 1] + 1;
        }
    }

    for i in (1..n as usize).rev() {
        if arr[i] < arr[i - 1] && can[i] >= can[i - 1] {
            can[i - 1] = can[i] + 1;
        }
    }
    can.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let arr_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        arr.push(arr_item);
    }

    let result = candies(n, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
