use std::fs;
use std::io::Read;

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn largest_palindrome_concat(a: &str, b: &str) -> String {
    let mut max_palindrome = String::new();

    for i in 0..a.len() {
        for j in i..a.len() {
            let sub_a = &a[i..=j];
            for k in 0..b.len() {
                for l in k..b.len() {
                    let sub_b = &b[k..=l];
                    let combined = format!("{}{}", sub_a, sub_b);
                    if is_palindrome(&combined) {
                        if combined.len() > max_palindrome.len() 
                            || (combined.len() == max_palindrome.len() && combined < max_palindrome) {
                            max_palindrome = combined;
                        }
                    }
                }
            }
        }
    }

    max_palindrome
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;

    let mut lines = input.lines();
    let t = lines.next().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let result = largest_palindrome_concat(&a, &b);

        println!("{result}");
    }
    Ok(())
}
