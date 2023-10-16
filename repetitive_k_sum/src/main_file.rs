use std::io::{self, BufRead};

fn comb(n: i64, mut k: i64) -> i64 {
    if k == 0 {
        return 1;
    } else if k > n / 2 {
        k = n - k;
    }

    let mut comb = n;
    for i in 2..=k {
        comb *= n - i + 1;
        comb /= i;
    }
    comb
}

fn clean_s(max_i: i64, mut sum: i64, mut rem_elems: i64, res: &Vec<i64>, s: &mut Vec<i64>) {
    if max_i == 0 {
        sum += rem_elems * res[0];
        rem_elems = 0;
    }

    if rem_elems == 0 {
        s.retain(|&x| x != sum);
    } else {
        for i in 0..rem_elems {
            clean_s(max_i - 1, sum + i * res[i as usize], rem_elems - i, &res, s);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let l = lines
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (n, k) = (l[0], l[1]);

        let mut s: Vec<i64> = Vec::new();

        let size = comb((n + k) as i64 - 1, n as i64 - 1);

        for (x, i) in lines
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
        {
            if x < size as usize {
                s.push(i);
            } else {
                break;
            }
        }

        s.sort();

        let mut res: Vec<i64> = Vec::new();
        res.push(s.first().unwrap() / k as i64);
        s.remove(0);

        for i in 1..n {
            res.push(s.first().unwrap() - res[0] * (k as i64 - 1));
            for j in 1..=k {
                clean_s(
                    i as i64 - 1,
                    j as i64 * res[i],
                    (k - j) as i64,
                    &res,
                    &mut s,
                );
            }
        }

        for i in res.iter() {
            print!("{i} ");
        }

        println!();
    }
}
