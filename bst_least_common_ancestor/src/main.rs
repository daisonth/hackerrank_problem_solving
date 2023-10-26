use std::fs;
use std::io::Read;

#[derive(Debug)]
enum BinaryTree {
    Empty,
    Node(Box<TreeNode>),
}

#[derive(Debug)]
struct TreeNode {
    value: usize,
    left: BinaryTree,
    right: BinaryTree,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree::Empty
    }

    fn insert(&mut self, value: usize) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node(Box::new(TreeNode {
                    value,
                    left: BinaryTree::new(),
                    right: BinaryTree::new(),
                }))
            }
            BinaryTree::Node(ref mut node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
        }
    }

    fn find_n(&mut self, value: usize, v: &mut Vec<usize>) {
        match *self {
            BinaryTree::Node(ref mut node) => {
                if node.value == value {
                    v.push(node.value);
                    return;
                } else if value < node.value {
                    v.push(node.value);
                    node.left.find_n(value, v);
                } else if value > node.value {
                    v.push(node.value);
                    node.right.find_n(value, v);
                }
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    lines.next();
    let nodes: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let i = lines
        .next()
        .unwrap()
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let v1 = i[0];
    let v2 = i[1];

    let mut tree: BinaryTree = BinaryTree::new();

    for i in nodes {
        tree.insert(i);
    }

    let mut vv1: Vec<usize> = Vec::new();
    tree.find_n(v1, &mut vv1);

    let mut vv2: Vec<usize> = Vec::new();
    tree.find_n(v2, &mut vv2);

    println!("vv1 : {vv1:?}");
    println!("vv2 : {vv2:?}");

    let mut res: usize = 0;
    let min = vv1.len().min(vv2.len());
    for _ in 0..min {
        let r1 = vv1.remove(0);
        let r2 = vv2.remove(0);

        if r1 == r2 {
            res = r1;
        }
    }
    println!("{}", res);

    Ok(())
}
