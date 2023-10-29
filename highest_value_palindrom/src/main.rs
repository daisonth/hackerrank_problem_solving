use std::fs;
use std::io::Read;

fn highest_value_palindrome(ss: &str, n: usize, mut k: i32) -> String {

    let mut s: Vec<char> = ss.chars().collect::<Vec<char>>();
    let mut a: Vec<i32> = vec![0; n / 2];

    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            s[i] = s[i].max(s[n - i - 1]);
            s[n - i - 1] = s[i].max(s[n - i - 1]);
            k -= 1;
            a[i] = 1;
        }
    }

    if k < 0 {
        println!("-1");
        "-1".to_string()
    } else if k == 0 {
        s.into_iter().collect()
    } else {
        for i in 0..n / 2 {
            if s[i] != '9' {
                if a[i] == 0 && k >= 1 {
                    s[i] = '9';
                    s[n - i - 1] = '9';
                    k -= 2;
                }

                if a[i] == 1 && k >= 1 {
                    s[i] = '9';
                    s[n - i - 1] = '9';
                    k -= 1;
                }
            }
        }

        if k >= 1 && n % 2 != 0 && s[n / 2] != '9' {
            s[n / 2] = '9';
            k -= 1;
        }

        s.into_iter().collect()
    }
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let v: Vec<usize> = lines
        .next()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (n, mut k): (usize, i32) = (v[0], v[1] as i32);

    let mut s: Vec<char> = lines
        .next()
        .unwrap()
        .trim_end()
        // .split_whitespace()
        .chars()
        // .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<char>>();

    let mut a: Vec<i32> = vec![0; n / 2];

    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            s[i] = s[i].max(s[n - i - 1]);
            s[n - i - 1] = s[i].max(s[n - i - 1]);
            k -= 1;
            a[i] = 1;
        }
    }

    if k < 0 {
        println!("-1");
    } else if k == 0 {
        for i in s {
            print!("{i}");
        }
        println!();
    } else {
        for i in 0..n / 2 {
            if s[i] != '9' {
                if a[i] == 0 && k >= 1 {
                    s[i] = '9';
                    s[n - i - 1] = '9';
                    k -= 2;
                }

                if a[i] == 1 && k >= 1 {
                    s[i] = '9';
                    s[n - i - 1] = '9';
                    k -= 1;
                }
            }
        }

        if k >= 1 && n % 2 != 0 && s[n / 2] != '9' {
            s[n / 2] = '9';
            k -= 1;
        }

        for i in s {
            print!("{i}");
        }
        println!();
    }

    Ok(())
}
