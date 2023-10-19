use std::fs;
use std::io::Read;

fn is_sorted(v: &Vec<i32>, len: usize) -> bool {
    for i in 1..len {
        if v[i] < v[i - 1] {
            return false;
        }
    }
    true
}

fn almost_sorted(arr: &[i32]) {
    let len = arr.len();
    let mut vec: Vec<i32> = arr.to_vec();

    if is_sorted(&vec, len) {
        println!("yes");
        return
    }

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut flag = false;

    for i in 1..len {
        if vec[i - 1] > vec[i] {
            if !flag {
                flag = true;
                x = i - 1;
            }
            y = i;
        }
    }

    vec.swap(x, y);
    if is_sorted(&vec, len) {
        println!("yes \nswap {} {}", x + 1, y + 1);
        return
    }

    vec.swap(x, y);
    let mut xx = x;
    let mut yy = y;
    while xx < yy {
        vec.swap(xx, yy);
        xx += 1;
        yy -= 1;
    }

    if is_sorted(&vec, len) {
        println!("yes \nreverse {} {}", x + 1, y + 1);
        return
    } else {
        println!("no");
    }
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    lines.next();
    let vec: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    almost_sorted(&vec);

    Ok(())
}

/*
* if array already sorted print :
*      yes
 *
 * if 2 elements can be swaped print :
 *      yes
 *      swap x y
 *
 * if a sub array needs to be reversed print :
 *      yes
 *      reverse x y
 *
 *  if we need both swap and reverse choose swap and print :
 *      yes
 *      swap x y
 *
 *  if we can't sort array with any of this then print :
 *      no
 */
