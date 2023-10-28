use std::fs;
use std::io::Read;

fn common_child(s1: &str, s2: &str) -> i32 {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[s1.len()][s2.len()]
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let a = lines.next().unwrap().trim_end();
    let b = lines.next().unwrap().trim_end();

    println!("{}", common_child(a, b));

    Ok(())
}
