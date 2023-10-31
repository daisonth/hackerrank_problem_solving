use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();
    let n = lines.next().unwrap().trim_end().parse::<usize>().unwrap();

    for _ in 0..n {
        let w: Vec<char> = lines
            .next()
            .unwrap()
            .trim_end()
            .chars()
            .collect::<Vec<char>>();
        let mut sw = w.clone();
        sw.sort();
        println!("orginal : {w:?}");
        println!("sorted : {sw:?}");
    }

    Ok(())
}
