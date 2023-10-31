// https://www.hackerrank.com/challenges/morgan-and-a-string

use std::fs;
use std::io::Read;

// function, morganAndString, takes two input strings 'a' and 'b' and merges them to create a new
// string. The goal is to make sure the new string is as small as possible when compared
// lexicographically.
//
// Here's how it works: It starts with the first characters of 'a' and 'b'. It picks the smaller
// character and adds it to the result 'r'. Then, it removes that character from its respective input
// string. This process continues until it's done. To ensure it can compare until the end, it adds a
// 'z' to both 'a' and 'b'. Finally, it removes any trailing 'z' and gives you the merged string 'r'.
// This way, it ensures the result is as small as possible while respecting the order of 'a' and 'b'.

fn morgan_and_string(a: &str, b: &str) -> String {
    let mut x = a.to_string() + "z";
    let mut y = b.to_string() + "z";
    let mut r = String::new();

    while !r.ends_with('z') {
        if x < y {
            r.push(x.chars().next().unwrap());
            x = (&x[1..].to_string()).clone();
        } else {
            r.push(y.chars().next().unwrap());
            y = (&y[1..].to_string()).clone();
        }
    }
    r.pop();
    r
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;

    let mut i = input.lines().into_iter();
    let t = i.next().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let a: &str = i.next().unwrap().trim_end();
        let b: &str = i.next().unwrap().trim_end();
        let result: String = morgan_and_string(&a, &b);

        println!("{}", result);
    }
    Ok(())
}
