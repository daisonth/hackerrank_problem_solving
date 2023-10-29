// use std::fs;
// use std::io::Read;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn dp(
    pos: usize,
    val: usize,
    modd: &i32,
    sieve: &[bool],
    cnt: &Vec<i32>,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if pos == 1001 {
        return sieve[val] as i32;
    }
    if memo[pos][val] != 1 {
        return memo[pos][val];
    }
    let numeven: i64 = cnt[pos + 3500] as i64 / 2;
    let numodd: i64 = cnt[pos + 3500] as i64 - numeven;
    if numodd > 0 {
        let res = dp(pos + 1, val ^ (pos + 3500), modd, sieve, cnt, memo);
        let rr: i32 = numodd as i32 * (res % modd);
        memo[pos][val] += rr;
        memo[pos][val] %= modd;
    }
    if numeven > 0 {
        let res = dp(pos + 1, val, modd, sieve, cnt, memo);
        let rr: i32 = numodd as i32 * (res % modd);
        memo[pos][val] += rr;
        memo[pos][val] %= modd;
    }

    memo[pos][val] += dp(pos + 1, val, modd, sieve, cnt, memo);
    memo[pos][val] %= modd;
    memo[pos][val]
}

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // let mut file = fs::File::open("./src/input")?;
    // let mut input: String = String::new();
    // file.read_to_string(&mut input)?;
    // let mut lines = input.lines().into_iter();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let modd: i32 = 1000000007;
    let mut cnt: Vec<i32> = vec![0; 4505];
    let mut memo: Vec<Vec<i32>> = vec![Vec::with_capacity(9000); 1005];

    let q: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .parse::<usize>()
        .unwrap();

    for i in 0..q {
        // let n: usize = lines.next().unwrap().trim_end().parse::<usize>().unwrap();
        lines.next();
        let mut arr: Vec<i32> = Vec::new();
        lines
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .for_each(|x| {
                arr.push(x.parse::<i32>().unwrap());
                cnt[arr[i] as usize] += 1;
            })
        // .map(|x| x.parse::<i32>().unwrap())
        // .collect::<Vec<i32>>();
    }

    let mut sieve: Vec<bool> = Vec::with_capacity(9000);
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..9000 {
        if sieve[i] {
            let mut j = i + i;
            while j < 9000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let res = dp(0, 0, &modd, &sieve, &cnt, &mut memo);
    writeln!(&mut fptr, "{}", res).ok();

    Ok(())
}
