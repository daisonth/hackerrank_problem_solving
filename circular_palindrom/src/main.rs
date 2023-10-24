use std::fs;
use std::io::Read;

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn len_of_largest_palindrom(s: &str, len: &mut usize) -> (usize, usize) {
    let mut i = 0;
    let mut j = 0;
    let vec: Vec<usize>;

    // if *len > s.len() / 2 {
    // vec = (0..(2 * s.len() - 1)).collect::<Vec<usize>>();
    // } else {
    //     vec = ((0..(2 * s.len() - 1)).rev()).collect::<Vec<usize>>();
    // }

    // for center in vec {
    for center in 0..(2 * s.len() - 1) {
        let (mut left, mut right) = (center / 2, center / 2 + center % 2);

        while right < s.len() && s.as_bytes()[left] == s.as_bytes()[right] {
            let p = s[left..=right].to_string();
            if p.len() > *len {
                *len = p.len();
                i = left;
                j = right + 1;
            } else {
            }
            if left == 0 || right == s.len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }

    (i, j)
}

fn circularpalindromes(s: &str) -> Vec<i32> {
    let n = s.len();
    let mut ss = s.to_string();
    let mut arr: Vec<i32> = Vec::with_capacity(ss.len());

    println!("n : {n}, ss: {ss}\n");

    let mut len = 0;
    let (mut i, mut j) = len_of_largest_palindrom(&ss, &mut len);
    println!("{i} to {j} = {}", &ss[i..j]);
    arr.push(len as i32);
    let ch = ss.remove(0);
    ss.push(ch);

    for q in 1..n {
        println!("\nloop {q}");
        println!("{i} : {j}");

        if i == 0 {
            if is_palindrome(&ss[n - len..n]) {}
        } else if j == n {
            if ss.chars().nth(i - 1) == ss.chars().last() {
                len += 1;
                i -= 1;
            }
        } else {
            len -= 2;
            j -= 1;
        }

        println!("{i} to {j} = {}", &ss[i..j]);

        let ch = ss.remove(0);
        ss.push(ch);
    }

    //     if i == 0 || j == 0 {
    //         len -= 2;
    //     }
    //
    //     (i, j) = len_of_largest_palindrom(&ss, &mut len);
    //     arr.push(len as i32);
    //
    //     let ch = ss.remove(0);
    //     ss.push(ch);
    // }

    arr
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;

    let mut lines = input.lines();
    lines.next();
    let s = lines.next().unwrap().trim();

    let arr = circularpalindromes(&s);
    for i in arr {
        println!("{i}");
    }

    Ok(())
}
