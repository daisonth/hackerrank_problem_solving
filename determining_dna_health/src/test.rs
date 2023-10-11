use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut health_store:Vec<i32> = Vec::new();

    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;


    let genes: Vec<&str> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let health: Vec<i32> = input
        .lines()
        .nth(2)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let s: usize = input.lines().nth(3).unwrap().parse::<usize>().unwrap();

    for line in input
        .lines()
        .rev()
        .take(s)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        let mut th: i32 = 0;
        let mut i = line.split_whitespace();
        let start = i.next().unwrap().parse::<usize>().unwrap();
        let end = i.next().unwrap().parse::<usize>().unwrap() + 1;
        let d = i.next().unwrap();

        for x in start..end {
            let num = d
                .chars()
                .collect::<Vec<char>>()
                .windows(genes[x].len())
                .filter(|w| w == &genes[x].chars().collect::<Vec<char>>())
                .count();

            if num > 0 {
                th += health[x as usize] * num as i32;
            }
        }
        health_store.push(th);
    }
    println!("{} {}",health_store.iter().min().unwrap(), health_store.iter().max().unwrap());
    Ok(())
}
