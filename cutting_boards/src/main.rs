use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn boardCutting(cost_y: &[i32], cost_x: &[i32]) -> i64 {
    let mut v: Vec<(i32, char)> = Vec::new();

    for i in 0..cost_y.len() {
        v.push((cost_y[i], 'h'));
    }

    for i in 0..cost_x.len() {
        v.push((cost_x[i], 'v'));
    }

    v.sort();
    let (mut min_cost, mut r, mut c): (i64, i64, i64) = (0, 1, 1);
    for i in 0..v.len() {
        if v[i].1 == 'h' {
            r += 1;
            min_cost = min_cost + v[i].0 as i64 * c;
        } else {
            c += 1;
            min_cost = min_cost + v[i].0 as i64 * r;
        }
    }

    min_cost % 1000000007
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let m = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let n = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let cost_y: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let cost_x: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = boardCutting(&cost_y, &cost_x);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
