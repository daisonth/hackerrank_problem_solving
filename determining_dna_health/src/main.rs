use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Default, Debug)]
struct TrieNode {
    indexes: Vec<usize>,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, index: usize, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.indexes.push(index);
    }
}

fn calculate_health(
    dna: &str,
    // genes: &[String],
    health: &[usize],
    start: usize,
    end: usize,
    trie: &Trie,
) -> i64 {
    let mut total_health: usize = 0;
    let d: Vec<char> = dna.chars().collect();
    for (i, _) in d.iter().enumerate() {
        let mut node = &trie.root;
        let mut j = i;
        while j < dna.len() && node.children.contains_key(&d[j]) {
            node = &node.children[&d[j]];
            for &index in &node.indexes {
                if index >= start && index <= end {
                    // if index >= start && index <= end && &genes[index] == &dna[i..=j] {
                    total_health += health[index];
                }
            }
            j += 1;
        }
    }
    total_health as i64
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();

    let genes: Vec<String> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let health: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let s: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .parse::<usize>()
        .unwrap();

    let mut trie: Trie = Trie::new();
    for (index, gene) in genes.iter().enumerate() {
        trie.insert(index, gene);
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for _ in 0..s {
        let strand: Vec<String> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|i| i.to_string())
            .collect();

        let start: usize = strand[0].parse::<usize>().unwrap();
        let end: usize = strand[1].parse::<usize>().unwrap();
        let dna: &str = &strand[2];

        // let total_health = calculate_health(dna, &genes, &health, start, end, &trie);
        let total_health = calculate_health(dna, &health, start, end, &trie);

        min = min.min(total_health);
        max = max.max(total_health);
    }
    println!("{} {}", min, max);
}
