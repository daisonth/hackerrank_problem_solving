use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut sea_level = 0;
    let mut valleys = 0;

    for character in path.chars() {
        match character {
            'U' => {
                sea_level += 1;
                if sea_level == 0 {
                    valleys += 1;
                }
            }
            'D' => {
                sea_level -= 1;
            }
            _ => {
                // Handle invalid characters (if needed)
            }
        }
    }

    valleys
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
