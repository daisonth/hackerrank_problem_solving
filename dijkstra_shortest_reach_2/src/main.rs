use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{Read, Result};

fn shortest_reach(n: i32, edges: &[Vec<i32>], s: i32) -> Vec<i32> {
    let start: usize = s as usize - 1;
    let mut vis: Vec<bool> = vec![false; n as usize]; // visited nodes
    let mut dist: Vec<i32> = vec![-1; n as usize]; // distances
    let mut pq = BinaryHeap::new(); // priority queue

    dist[start] = 0;
    pq.push(Reverse((0, start as i32)));

    let mut adjl: Vec<Vec<(i32, i32)>> = vec![Vec::new(); n as usize]; // adjacency list of weighted graph
    for edge in edges {
        adjl[edge[0] as usize - 1].push((edge[1] - 1, edge[2]));
        adjl[edge[1] as usize - 1].push((edge[0] - 1, edge[2]));
    }

    while !pq.is_empty() {
        let (minvalue, index) = pq.pop().unwrap().0;
        let i: usize = index as usize;
        vis[i] = true;
        if dist[i] != -1 && dist[i] < minvalue {
            continue;
        }
        for edge in adjl[i].iter() {
            let e = edge.0 as usize;
            if vis[e] {
                continue;
            } else {
                let new_dist = dist[i] + edge.1;
                if dist[e] == -1 || new_dist < dist[e] {
                    dist[e] = new_dist;
                    pq.push(Reverse((new_dist, edge.0)));
                }
            }
        }
    }
    dist.remove(start);

    dist
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

    println!("{:?}", shortest_reach(n, &edges, start));

    Ok(())
}
