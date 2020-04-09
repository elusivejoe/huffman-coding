use crate::huffman;
use std::collections::HashMap;

pub mod tree;

pub fn calc_codes(root_node: &Box<huffman::Node>) -> HashMap<u8, huffman::HuffmanCode> {
    let mut codes = HashMap::<u8, huffman::HuffmanCode>::new();
    let mut current_code = huffman::HuffmanCode::new();

    walk_tree(root_node, &mut codes, &mut current_code);

    codes
}

pub fn compress(bytes: &[u8], codes: &HashMap<u8, huffman::HuffmanCode>) -> (Vec<u8>, usize) {
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

fn walk_tree(
    node: &Box<huffman::Node>,
    codes: &mut HashMap<u8, huffman::HuffmanCode>,
    current_code: &mut huffman::HuffmanCode,
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

#[cfg(test)]
mod tests {
    use crate::{compressor, huffman};
    use std::collections::BinaryHeap;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    fn initial_tree() -> BinaryHeap<Box<huffman::Node>> {
        let mut bottom_leaves = BinaryHeap::<Box<huffman::Node>>::new();

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 2,
            byte: Some('A' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 2,
            byte: Some('B' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 2,
            byte: Some('C' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 3,
            byte: Some('D' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 5,
            byte: Some('F' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves.push(Box::new(huffman::Node {
            frequency: 10,
            byte: Some('E' as u8),
            left: None,
            right: None,
        }));

        bottom_leaves
    }

    #[test]
    fn test_compress() {
        let initial_tree = initial_tree();
        let huffman_tree = compressor::tree::build(initial_tree);
        let codes = compressor::calc_codes(&huffman_tree.expect("No empty tree possible."));

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

        let res = compressor::compress(message.as_bytes(), &codes);

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
}
