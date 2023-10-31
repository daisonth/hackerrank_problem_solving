use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn prims(num: i32, edges: &[Vec<i32>], start: i32) -> usize {
    let n = num as usize; // number of nodes in the graph
    let s = start as usize - 1; // make starting node 0 (for easy indexing)
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

    vis[s] = true; // mark starting node as visited
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


fn main() {
    //read input from file called "input"
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

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

    let start = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = prims(n, &edges, start);

    writeln!(&mut fptr, "{}", result).ok();
}
