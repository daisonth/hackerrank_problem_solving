use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();
    let n = lines.next().unwrap().trim_end().parse::<usize>().unwrap();

    for _ in 0..n {
        let s = lines.next().unwrap().trim_end().to_string();
        // let w: Vec<char> = lines
        //     .next()
        //     .unwrap()
        //     .trim_end()
        //     .chars()
        //     .collect::<Vec<char>>();
        // let mut sw = w.as_bytes();
        // sw.sort();
        // println!("orginal : {w:?}");
        // println!("bytes : {sw:?}\n");
        // let s = input.trim().to_string();

        let mut l = s.clone();
        let mut s_chars: Vec<char> = s.chars().collect();

        if next_permutation(&mut s_chars) {
            let s = s_chars.iter().collect::<String>();
            if s <= l {
                println!("no answer");
            } else {
                println!("{}", s);
            }
        } else {
            println!("no answer");
        }
    }

    Ok(())
}
