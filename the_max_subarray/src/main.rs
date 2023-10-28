// https://www.hackerrank.com/challenges/maxsubarray/problem

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn max_subarray(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();

    let mut dp = vec![[0; 2]; n];
    dp[0][0] = arr[0];
    dp[0][1] = arr[0];

    for i in 1..n {
        dp[i][0] = std::cmp::max(dp[i - 1][0] + arr[i], arr[i]);
        dp[i][1] = std::cmp::max(std::cmp::max(dp[i - 1][1], dp[i - 1][1] + arr[i]), arr[i]);
    }

    let v: Vec<i32> = vec![dp.iter().map(|&x| x[0]).max().unwrap(), dp[n - 1][1]];
    v
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let arr: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = max_subarray(&arr);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
