use std::fs;
use std::io::Read;

fn get_substring(s: &String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let c: Vec<char> = s.chars().collect();

    let l = s.len();
    for i in 1..=l {
        for w in c.windows(i) {
            let r: String = w.iter().collect();
            v.push(r);
        }
    }
    v
}

fn find_strings(w: &[String], queries: &[i32]) -> Vec<String> {
    let mut s: Vec<String> = Vec::new();
    let mut substr: Vec<String> = Vec::new();
    for i in w {
        substr.append(&mut get_substring(&i));
    }
    substr.sort();
    substr.dedup();
    let l = substr.len();

    // println!("substrings: {substr:?}");

    for i in queries {
        if *i - 1 < l as i32 {
            s.push(substr[*i as usize - 1].clone());
        } else {
            s.push("INVALID".to_string());
        }
    }
    s
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut w: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        w.push(lines.next().unwrap().trim_end().to_string());
    }

    let q = lines.next().unwrap().trim().parse::<usize>().unwrap();

    let mut queries: Vec<i32> = Vec::with_capacity(q);
    for _ in 0..q {
        queries.push(lines.next().unwrap().trim_end().parse::<i32>().unwrap());
    }

    let result: Vec<String> = find_strings(&w, &queries);
    for s in result {
        println!("{s}");
    }

    Ok(())
}
