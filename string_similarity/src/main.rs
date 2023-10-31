use std::fs;
use std::io::Read;

// function, 'stringSimilarity', figures out how much the beginning of a word is similar to its
// suffixes. It does this by checking the number of characters at the start of the word that are
// also at the start of its different suffixes. The more characters they have in common, the higher
// the similarity. It adds up these similarities for all suffixes of the word and returns the total
// similarity score. 

fn string_similarity(s: &str) -> i64 {
    // convert string 's' into bytes for efficient character access.
    let v = s.as_bytes();

    let len = v.len();
    let mut n: usize = 1;
    let mut sum: i64 = len as i64;

    for i in 1..len {
        if v[0] == v[i] {
            sum += i as i64;
        } else {
            n = i;
            break;
        }
    }

    if n != len - 1 {
        for i in n..len {
            let (mut p, mut q) = (i, 0);
            while p < len && v[q] == v[p] {
                sum += 1;
                q += 1;
                p += 1;
            }
        }
    }
    sum
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        println!("{}", string_similarity(lines.next().unwrap().trim_end()));
    }

    Ok(())
}
