use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{Read, Result};

fn lazy_prims(num: i32, edges: &[Vec<i32>], start: i32) -> usize {
    let n = num as usize;
    let s = start as usize - 1;
    let mut pq = BinaryHeap::new(); // priority queue
    let mut vis: Vec<bool> = vec![false; n as usize]; // visited nodes
    let mut g: Vec<Vec<(i32, i32)>> = vec![Vec::new(); n as usize]; // adjacency list of weighted graph
    for edge in edges {
        g[edge[0] as usize - 1].push((edge[1] - 1, edge[2]));
        g[edge[1] as usize - 1].push((edge[0] - 1, edge[2]));
    }

    let m = n - 1;
    let (mut edgecount, mut mstcost) = (0, 0);
    let mut mstedges: Vec<usize> = vec![0; m];

    vis[s] = true;
    for edge in g[s].iter() {
        if !vis[edge.0 as usize] {
            pq.push(Reverse((edge.1 as usize, s, edge.0 as usize)));
        }
    }

    while !pq.is_empty() && edgecount != m {
        let edge = pq.pop().unwrap().0;

        if vis[edge.2] {
            continue;
        }

        mstedges[edgecount] = edge.0;
        edgecount += 1;
        mstcost += edge.0;

        vis[edge.2] = true;
        for edge in g[edge.2].iter() {
            if !vis[edge.0 as usize] {
                pq.push(Reverse((edge.1 as usize, s, edge.0 as usize)));
            }
        }
    }

    mstcost
}

fn main() -> Result<()> {
    let mut file = File::open("./src/input").unwrap();
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let mut line = lines.next().unwrap().trim_end().split_whitespace();
    let n = line.next().unwrap().parse::<i32>().unwrap();
    let m = line.next().unwrap().parse::<i32>().unwrap();
    let mut edges: Vec<Vec<i32>> = Vec::new();

    for _ in 0..m {
        edges.push(
            lines
                .next()
                .unwrap()
                .trim_end()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let start = lines.next().unwrap().trim_end().parse::<i32>().unwrap();

    println!("{:?}", lazy_prims(n, &edges, start));

    Ok(())
}
