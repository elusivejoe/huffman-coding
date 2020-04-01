use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;

mod utils;

#[derive(Eq)]
struct Node {
    frequency: u64,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

//note: reversed order
impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

fn collect_bottom_leaves<T: Read + Seek>(reader: &mut T) -> std::io::Result<BinaryHeap<Box<Node>>> {
    let mut byte = [0u8];
    let stream_len = utils::stream_length(reader)?;

    let mut frequencies = HashMap::<u8, u64>::new();

    while utils::stream_current_position(reader)? != stream_len {
        reader.read_exact(&mut byte)?;

        match frequencies.get_mut(&byte[0]) {
            Some(frequency) => *frequency += 1,
            None => {
                frequencies.insert(byte[0], 1);
                ()
            }
        }
    }

    let mut result = BinaryHeap::<Box<Node>>::new();

    for entry in frequencies {
        result.push(Box::new(Node {
            byte: Some(entry.0),
            frequency: entry.1,
            left: None,
            right: None,
        }));
    }

    Ok(result)
}

fn walk_tree(node: &Box<Node>, codes: &mut HashMap<u8, String>, current_code: &mut String) {
    if let Some(byte) = node.byte {
        codes.insert(byte, current_code.clone());
        return;
    }

    if let Some(left) = &node.left {
        current_code.push('0');
        walk_tree(left, codes, current_code);
        current_code.pop();
    }

    if let Some(right) = &node.right {
        current_code.push('1');
        walk_tree(right, codes, current_code);
        current_code.pop();
    }
}

fn calc_codes(root_node: &Box<Node>) {
    let mut codes = HashMap::<u8, String>::new();
    let mut current_code = String::new();

    walk_tree(root_node, &mut codes, &mut current_code);

    for entry in &codes {
        match codes.get(entry.0) {
            Some(code) => {
                println!("{}: {}", *entry.0 as char, code);
            }
            None => panic!("Broken tree."),
        }
    }
}

fn build_tree(mut bin_heap: BinaryHeap<Box<Node>>) -> Option<Box<Node>> {
    while bin_heap.len() > 1 {
        let right_node = bin_heap.pop().unwrap();
        let left_node = bin_heap.pop().unwrap();

        let joined_node = Node {
            frequency: left_node.frequency + right_node.frequency,
            byte: None,
            left: Some(left_node),
            right: Some(right_node),
        };

        bin_heap.push(Box::new(joined_node));
    }

    bin_heap.pop()
}

fn main() -> std::io::Result<()> {
    let args = utils::parse_args(&env::args().collect())?;

    println!(
        "Mode: {}\nInput: {}\nOutput{}\n",
        args.mode, args.file_in, args.file_out
    );

    let file_in = File::open(Path::new(&args.file_in))?;
    let mut reader = BufReader::new(file_in);

    let initial_tree = collect_bottom_leaves(&mut reader)?;
    let huffman_tree = build_tree(initial_tree);

    if let Some(root_node) = huffman_tree {
        calc_codes(&root_node);
    } else {
        println!("Empty tree.");
    }

    Ok(())
}
