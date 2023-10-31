use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{Read, Result};

// The `lazy_prims` function implements the lazy version of Prim's algorithm for finding
// the minimum spanning tree (MST) in a weighted graph. It takes as input the number of nodes
// `num`, a list of edges represented as tuples with their endpoints and weights, and a starting
// node `start`. The function initializes a priority queue (`pq`) to track edges, a boolean
// array(vis) to mark visited nodes, and an adjacency list(g) to represent the weighted graph. It starts
// from the specified node, continually adds the minimum-weight edge connecting the MST to the
// unvisited nodes, and updates the priority queue accordingly. This process continues until the
// MST has n-1 edges (where n is the number of nodes). The function returns the total cost of the
// MST. In summary, the code constructs an MST in a graph by iteratively selecting the shortest
// edge connected to the growing MST.

fn lazy_prims(num: i32, edges: &[Vec<i32>], start: i32) -> usize {
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

fn main() -> Result<()> {
    //read input from file called "input"
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
