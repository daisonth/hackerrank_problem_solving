use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let t = lines.next().unwrap().trim_end().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut v = lines.next().unwrap().trim_end().split_whitespace();
        let (n, a, b): (usize, usize, usize) = (
            v.next().unwrap().parse().unwrap(),
            v.next().unwrap().parse().unwrap(),
            v.next().unwrap().parse().unwrap(),
        );

        let s = lines.next().unwrap().trim_end();
    }
    Ok(())
}
