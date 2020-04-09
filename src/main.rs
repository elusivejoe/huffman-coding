use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

mod utils;

#[derive(Eq)]
struct Node {
    frequency: u64,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Copy, Clone)]
struct HuffmanCode {
    bin_repres: u8,
    bin_length: u8,
}

impl HuffmanCode {
    fn new() -> HuffmanCode {
        HuffmanCode {
            bin_repres: 0,
            bin_length: 0,
        }
    }

    fn push_bit(&mut self, set: bool) {
        assert!(self.bin_length < 8, "Attempted to push more than 8 bits.");

        self.bin_repres <<= 1;

        if set {
            self.bin_repres |= 0b00000001;
        }

        self.bin_length += 1;
    }

    fn pop_bit(&mut self) -> bool {
        assert!(self.bin_length > 0, "Attempted to extract -1 bit.");

        let popped = (self.bin_repres & 0b00000001) == 1;

        self.bin_repres >>= 1;
        self.bin_length -= 1;

        popped
    }

    fn bin_repres(&self) -> u8 {
        self.bin_repres
    }

    fn bin_length(&self) -> u8 {
        self.bin_length
    }
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

fn walk_tree(
    node: &Box<Node>,
    codes: &mut HashMap<u8, HuffmanCode>,
    current_code: &mut HuffmanCode,
) {
    if let Some(byte) = node.byte {
        codes.insert(byte, *current_code);
        return;
    }

    if let Some(left) = &node.left {
        current_code.push_bit(false);
        walk_tree(left, codes, current_code);
        current_code.pop_bit();
    }

    if let Some(right) = &node.right {
        current_code.push_bit(true);
        walk_tree(right, codes, current_code);
        current_code.pop_bit();
    }
}

fn calc_codes(root_node: &Box<Node>) -> HashMap<u8, HuffmanCode> {
    let mut codes = HashMap::<u8, HuffmanCode>::new();
    let mut current_code = HuffmanCode::new();

    walk_tree(root_node, &mut codes, &mut current_code);

    codes
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

fn compress(bytes: &[u8], codes: &HashMap<u8, HuffmanCode>) -> (Vec<u8>, usize) {
    let mut result = Vec::<u8>::new();

    let mut total_length = 0usize;

    for byte in bytes {
        let code = codes
            .get(byte)
            .expect("The tree must contain all possible variants.");

        let free_bits = (result.len() * 8 - total_length) as u8;

        if free_bits == 0 {
            let packed_byte = code.bin_repres() << (8 - code.bin_length());
            result.push(packed_byte);
        } else {
            let tail = result.pop().expect("Broken compressed sequence.");

            if free_bits >= code.bin_length() {
                let tail_filler = code.bin_repres() << (free_bits - code.bin_length());
                let packed_byte = tail | tail_filler;

                result.push(packed_byte);
            } else {
                let tail_filler = code.bin_repres() >> (code.bin_length() - free_bits);
                let packed_byte = tail | tail_filler;

                result.push(packed_byte);

                let new_tail = code.bin_repres() << (8 - (code.bin_length() - free_bits));

                result.push(new_tail);
            }
        }

        total_length += code.bin_length() as usize;
    }

    (result, total_length)
}

fn is_bit_set(byte: u8, bit_num: u8) -> Option<bool> {
    if bit_num >= 8 {
        return None;
    }

    let mask = 0b10000000 >> bit_num;

    Some(byte & mask > 0)
}

fn check_bit_set(stream: &[u8], bit_num: usize) -> Option<bool> {
    if bit_num >= stream.len() * 8 {
        return None;
    }

    let byte_idx = bit_num / 8;
    let relative_bit_idx = bit_num % 8;

    is_bit_set(stream[byte_idx], relative_bit_idx as u8)
}

fn decompress(compressed_stream: &(Vec<u8>, usize), huffman_tree_root: &Box<Node>) -> String {
    let compr_data = &compressed_stream.0;
    let compr_data_bin_len = compressed_stream.1;

    let mut result = String::new();
    let mut current_node = huffman_tree_root;

    for index in 0..compr_data_bin_len {
        if check_bit_set(compr_data, index).expect("Wrong bit num") {
            if let Some(right_node) = &current_node.right {
                current_node = right_node;
            }
        } else {
            if let Some(left_node) = &current_node.left {
                current_node = left_node;
            }
        }

        if let Some(byte) = &current_node.byte {
            current_node = huffman_tree_root;
            result.push(*byte as char);
        }
    }

    result
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
        let codes = calc_codes(&root_node);
        let stream_len = utils::stream_length(&mut reader)?;

        reader.seek(SeekFrom::Start(0))?;

        while utils::stream_current_position(&mut reader)? != stream_len {
            let chunk_size = std::cmp::min(stream_len, 1024);

            let mut chunk = vec![0u8; chunk_size as usize];
            reader.read_exact(&mut chunk)?;

            let res = compress(&chunk, &codes);
            println!(">> Compressed stream >>");
            for entry in &res.0 {
                print!("{:b} ", entry);
            }
            println!("\n<< Compressed stream <<");

            println!(">> Decompressed stream >>");
            println!("{}", decompress(&res, &root_node));
            println!("<< Decompressed stream <<\n");

            println!(
                "Message size: {} Compressed size: {}",
                chunk.len(),
                res.0.len()
            );
        }
    } else {
        println!("Empty tree.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        build_tree, calc_codes, check_bit_set, compress, decompress, is_bit_set, HuffmanCode, Node,
    };
    use std::collections::BinaryHeap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_huff_code_operations() {
        let mut huff_code = HuffmanCode::new();

        huff_code.push_bit(true);
        assert_eq!(huff_code.bin_length(), 1);
        assert_eq!(huff_code.bin_repres(), 0b00000001);

        huff_code.push_bit(false);
        assert_eq!(huff_code.bin_length(), 2);
        assert_eq!(huff_code.bin_repres(), 0b00000010);

        huff_code.push_bit(true);
        assert_eq!(huff_code.bin_length(), 3);
        assert_eq!(huff_code.bin_repres(), 0b00000101);

        huff_code.push_bit(true);
        assert_eq!(huff_code.bin_length(), 4);
        assert_eq!(huff_code.bin_repres(), 0b00001011);

        huff_code.push_bit(false);
        assert_eq!(huff_code.bin_length(), 5);
        assert_eq!(huff_code.bin_repres(), 0b00010110);

        huff_code.push_bit(true);
        assert_eq!(huff_code.bin_length(), 6);
        assert_eq!(huff_code.bin_repres(), 0b00101101);

        huff_code.push_bit(true);
        assert_eq!(huff_code.bin_length(), 7);
        assert_eq!(huff_code.bin_repres(), 0b01011011);

        huff_code.push_bit(false);
        assert_eq!(huff_code.bin_length(), 8);
        assert_eq!(huff_code.bin_repres(), 0b10110110);

        //now, in reverse order

        assert_eq!(huff_code.pop_bit(), false);
        assert_eq!(huff_code.bin_length(), 7);
        assert_eq!(huff_code.bin_repres(), 0b01011011);

        assert_eq!(huff_code.pop_bit(), true);
        assert_eq!(huff_code.bin_length(), 6);
        assert_eq!(huff_code.bin_repres(), 0b00101101);

        assert_eq!(huff_code.pop_bit(), true);
        assert_eq!(huff_code.bin_length(), 5);
        assert_eq!(huff_code.bin_repres(), 0b00010110);

        assert_eq!(huff_code.pop_bit(), false);
        assert_eq!(huff_code.bin_length(), 4);
        assert_eq!(huff_code.bin_repres(), 0b00001011);

        assert_eq!(huff_code.pop_bit(), true);
        assert_eq!(huff_code.bin_length(), 3);
        assert_eq!(huff_code.bin_repres(), 0b00000101);

        assert_eq!(huff_code.pop_bit(), true);
        assert_eq!(huff_code.bin_length(), 2);
        assert_eq!(huff_code.bin_repres(), 0b00000010);

        assert_eq!(huff_code.pop_bit(), false);
        assert_eq!(huff_code.bin_length(), 1);
        assert_eq!(huff_code.bin_repres(), 0b00000001);

        assert_eq!(huff_code.pop_bit(), true);
        assert_eq!(huff_code.bin_length(), 0);
        assert_eq!(huff_code.bin_repres(), 0b00000000);
    }

    fn test_bottom_leaves() -> BinaryHeap<Box<Node>> {
        let mut bottom_leaves = BinaryHeap::<Box<Node>>::new();

        bottom_leaves.push(Box::new(Node {
            frequency: 2,
            byte: Some('A' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(Node {
            frequency: 2,
            byte: Some('B' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(Node {
            frequency: 2,
            byte: Some('C' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(Node {
            frequency: 3,
            byte: Some('D' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(Node {
            frequency: 5,
            byte: Some('F' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(Node {
            frequency: 10,
            byte: Some('E' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves
    }

    #[test]
    fn test_compress() {
        let initial_tree = test_bottom_leaves();
        let huffman_tree = build_tree(initial_tree);
        let codes = calc_codes(&huffman_tree.expect("No empty tree possible."));

        let file_in = File::open(Path::new("test-data/dataset_0/expected.txt"));

        let message = match file_in {
            Ok(mut file) => {
                let mut str = String::new();
                file.read_to_string(&mut str).unwrap();
                str
            }
            Err(_) => {
                assert!(false, "Could not open a test dataset");
                String::new()
            }
        };

        let res = compress(message.as_bytes(), &codes);

        let expected = (
            vec![
                0b00100010u8,
                0b00000000,
                0b00000000,
                0b01100110,
                0b11111111,
                0b11110110,
                0b10010010,
                0b10000000,
            ],
            57usize,
        );

        assert_eq!(res, expected);
    }

    #[test]
    fn test_decompress() {
        let file_in = File::open(Path::new("test-data/dataset_0/expected.txt"));

        let expected_message = match file_in {
            Ok(mut file) => {
                let mut str = String::new();
                file.read_to_string(&mut str).unwrap();
                str
            }
            Err(_) => {
                assert!(false, "Could not open a test dataset");
                String::new()
            }
        };

        let compressed_message = (
            vec![
                0b00100010u8,
                0b00000000,
                0b00000000,
                0b01100110,
                0b11111111,
                0b11110110,
                0b10010010,
                0b10000000,
            ],
            57usize,
        );

        let initial_tree = test_bottom_leaves();
        let huffman_tree = build_tree(initial_tree);

        assert_eq!(
            decompress(
                &compressed_message,
                &huffman_tree.expect("No empty tree possible.")
            ),
            expected_message
        );
    }

    #[test]
    fn test_bit_set() {
        let byte: u8 = 0b10110101;
        assert_eq!(is_bit_set(byte, 0), Some(true));
        assert_eq!(is_bit_set(byte, 1), Some(false));
        assert_eq!(is_bit_set(byte, 2), Some(true));
        assert_eq!(is_bit_set(byte, 3), Some(true));
        assert_eq!(is_bit_set(byte, 4), Some(false));
        assert_eq!(is_bit_set(byte, 5), Some(true));
        assert_eq!(is_bit_set(byte, 6), Some(false));
        assert_eq!(is_bit_set(byte, 7), Some(true));

        assert_eq!(is_bit_set(byte, 8), None);
        assert_eq!(is_bit_set(byte, 42), None);
    }

    #[test]
    fn test_check_bit() {
        let stream = vec![0b10000000u8, 0b00000001, 0b00000000, 0b10100001];

        assert_eq!(check_bit_set(&stream, 0), Some(true));
        assert_eq!(check_bit_set(&stream, 7), Some(false));
        assert_eq!(check_bit_set(&stream, 14), Some(false));
        assert_eq!(check_bit_set(&stream, 15), Some(true));
        assert_eq!(check_bit_set(&stream, 24), Some(true));
        assert_eq!(check_bit_set(&stream, 25), Some(false));
        assert_eq!(check_bit_set(&stream, 26), Some(true));
        assert_eq!(check_bit_set(&stream, 27), Some(false));
        assert_eq!(check_bit_set(&stream, 31), Some(true));

        assert_eq!(check_bit_set(&stream, 32), None);
        assert_eq!(check_bit_set(&stream, 42), None);
    }
}
