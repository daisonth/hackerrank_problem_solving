use std::collections::HashMap;

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
    genes: &[&str],
    health: &[usize],
    start: usize,
    end: usize,
    trie: &Trie,
) -> i32 {
    let mut total_health: usize = 0;
    println!("dna : {}", dna.to_string());

    for (i, c) in dna.chars().enumerate() {
        let mut node = &trie.root;
        let mut j = i;
        while j < dna.len() {
            match node.children.get(&c) {
                Some(n) => {
                    node = n;
                    for &index in &node.indexes {
                        if index >= start && index <= end && genes[index] == &dna[i..=j] {
                            total_health += health[index];
                        }
                    }
                }
                None => break,
            }
            j += 1;
        }
    }
    total_health as i32
}

fn main() {
    let mut trie: Trie = Trie::new();

    let genes: Vec<&str> = "a b c aa d b".split_whitespace().collect::<Vec<&str>>();

    let health: Vec<usize> = "1 2 3 4 5 6"
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut strand = "1 5 caaab".split_whitespace().into_iter();

    for (index, gene) in genes.iter().enumerate() {
        trie.insert(index, gene);
        println!("{index}, {gene}");
    }

    let start: usize = strand.next().unwrap().parse::<usize>().unwrap();
    let end: usize = strand.next().unwrap().parse::<usize>().unwrap();
    let dna: &str = strand.next().unwrap();

    let total_health = calculate_health(dna, &genes, &health, start, end, &trie);

    println!("total health : {}", total_health);
}
