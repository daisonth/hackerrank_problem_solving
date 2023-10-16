use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn find_sum(
    k: usize,
    v: &mut Vec<usize>,
    prefix: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
    // result: &mut Vec<usize>,
    map: &mut HashMap<usize, Vec<Vec<usize>>>,
) {
    if prefix.len() == k {
        // println!("k = {k}");
        let sum = &prefix.iter().sum();
        if map.contains_key(sum) {
            map.get_mut(sum).unwrap().push((*prefix).clone());
            // result.push((*prefix).clone());
        }
        return;
    }

    for i in 0..v.len() {
        let mut new_prefix = prefix.clone();
        new_prefix.push(v[i] as usize);
        find_sum(k, v, &mut new_prefix, result, map);
    }
}

fn generate(
    n: usize,
    k: usize,
    start: i64,
    end: i64,
    map: &mut HashMap<usize, Vec<Vec<usize>>>,
    prefix: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
    r: &mut Vec<Vec<usize>>,
) {
    if prefix.len() == n {
        prefix.sort();
        prefix.dedup();
        result.push((*prefix).clone());
        return;
    }

    for i in start..=end {
        let mut new_prefix = prefix.clone();
        new_prefix.push(i as usize);
        generate(n, k, start, end, map, &mut new_prefix, result, r);
    }

    result.sort();
    result.dedup();
    for i in result.iter_mut() {
        let mut v: Vec<usize> = Vec::new();
        find_sum(k, i, &mut v, r, map);
    }
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut l = lines.next().unwrap().trim_end().split_whitespace();
        let n = l.next().unwrap().parse::<usize>().unwrap();
        let k = l.next().unwrap().parse::<usize>().unwrap();
        println!("n ={n}, k ={k}");

        let mut map: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        let mut items: Vec<usize> = Vec::new();

        let mut min = i64::MAX;
        let mut max = i64::MIN;

        for i in lines
            .next()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
        {
            min = min.min(i as i64);
            max = max.max(i as i64);
            items.push(i);
            map.insert(i, Vec::new());
        }

        let start = min / k as i64;
        let end = max / k as i64;

        let mut prefix: Vec<usize> = Vec::new();
        let mut result: Vec<Vec<usize>> = Vec::new();
        let mut r: Vec<Vec<usize>> = Vec::new();

        generate(n, k, start, end, &mut map, &mut prefix, &mut result, &mut r);

        for i in items.iter() {
            if map.contains_key(i) {
                map.get_mut(i).unwrap().iter_mut().for_each(|x| {
                    x.sort();
                    x.dedup();
                });
                map.get_mut(i).unwrap().sort();
                map.get_mut(i).unwrap().dedup();
                println!("{:?}", map.get(i).unwrap());
            }
            // println!();
        }

        // r.sort();
        // r.dedup();
        // println!("{:?}", r);

        println!();
    }
    Ok(())
}
