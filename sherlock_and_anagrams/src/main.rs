use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn sherlock_and_anagrams(s: &str) -> i32 {
    let vec = s.as_bytes();
    let mut count: usize = 0;
    let mut map: HashMap<Vec<u8>, usize> = HashMap::new();

    let len = vec.len();
    for i in 1..len {
        for j in vec.windows(i) {
            let mut k: Vec<u8> = j.to_vec();
            k.sort();
            if map.contains_key(&k) {
                let v = map.get(&k).unwrap();
                map.insert(k, v + 1);
            } else {
                map.insert(k, 1);
            }
        }
    }

    for (_, value) in map.iter() {
        if value > &1 {
            for i in 1..*value {
                count += value - i;
            }
        }
    }
    count as i32
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let str = lines.next().unwrap().trim_end();
        println!("{}", sherlock_and_anagrams(&str));
    }

    Ok(())
}
