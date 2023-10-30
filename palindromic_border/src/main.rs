use std::fs;
use std::io::Read;

struct Node {
    suff: i32,
    l: i32,
    c: [i32; 26],
    cnt: i32,
}

fn getsuff(i: usize, mut x: usize, b: &Vec<Node>, a: &Vec<char>) -> usize {
    while ((i - 1 - b[x].l as usize) < 0) || (a[i - 1 - b[x].l as usize] != a[i]) {
        x = b[x].suff as usize;
    }
    return x;
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    const N: usize = 100000;
    let mut b: Vec<Node> = Vec::with_capacity(N + 2);

    // let x = lines.next().unwrap().trim().parse::<i32>().unwrap();

    b[0].suff = 1;
    b[0].l = 0;
    b[1].suff = 1;
    b[1].l = -1;

    let a: Vec<char> = lines
        .next()
        .unwrap()
        .trim_end()
        .chars()
        .collect::<Vec<char>>();

    let (mut x, mut y): (usize, usize) = (1, 2);

    for i in 0..a.len() {
        x = getsuff(i, x, &b, &a);
        let f = a[i];
        if !b[x].c[f - 'a' as u8] {}
    }

    Ok(())
}
