use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortestReach(n: i32, edges: &[Vec<i32>], s: i32) -> Vec<i32> {
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

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut edges: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

        for i in 0..m as usize {
            edges.push(Vec::with_capacity(3_usize));

            edges[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let s = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = shortestReach(n, &edges, s);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
