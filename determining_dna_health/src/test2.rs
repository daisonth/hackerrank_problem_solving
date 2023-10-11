use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let genes: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let health: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let s = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut h: i32 = 0;
    let mut l: i32 = 0;
    for x in 0..s {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let mut th: i32 = 0;
        let start = first_multiple_input[0].trim().parse::<usize>().unwrap();
        let end = first_multiple_input[1].trim().parse::<usize>().unwrap() + 1;
        let d = &first_multiple_input[2];

        for x in start..end {
            let num = d
                .chars()
                .collect::<Vec<char>>()
                .windows(genes[x].len())
                .filter(|w| w == &genes[x].chars().collect::<Vec<char>>())
                .count();

            if num > 0 {
                th += health[x as usize] * num as i32;
            }
        }
        if x == 0 {
            l = th;
        }
        if th < l {
            l = th;
        }
        if th > h {
            h = th;
        }
    }
    println!("{} {}", l, h);
}
