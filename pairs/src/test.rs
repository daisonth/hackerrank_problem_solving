use std::fs;
use std::io::Read;

// function, 'pairs', counts the number of pairs in an array of integers that have a specific
// target difference 'k'. It achieves this by sorting the input array, iterating through each pair
// of elements, and checking if their difference equals 'k'. If a pair with the desired difference
// is found, it increments a count. The function then returns the count, which represents the
// number of pairs with the specified difference 'k' in the array.

fn pairs(k: i32, arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut a = arr.to_vec();
    let l = a.len();
    a.sort();

    for i in 0..l {
        for j in 1..l {
            if a[j] - a[i] == k {
                sum += 1;
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

    let mut ln = lines.next().unwrap().trim_end().split_whitespace();
    ln.next();

    let diff = ln.next().unwrap().parse::<i32>().unwrap();
    let arr = lines
        .next()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{}", pairs(diff, &arr));

    Ok(())
}
