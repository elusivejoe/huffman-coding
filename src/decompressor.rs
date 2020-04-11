use crate::{bin_operations, huffman};

pub fn decompress(
    compressed_stream: &(Vec<u8>, usize),
    huffman_tree_root: &Box<huffman::Node>,
) -> String {
    let compr_data = &compressed_stream.0;
    let compr_data_bin_len = compressed_stream.1;

    let mut result = String::new();
    let mut current_node = huffman_tree_root;

    for index in 0..compr_data_bin_len {
        if bin_operations::check_bit_set(compr_data, index).expect("Wrong bit num") {
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

#[cfg(test)]
mod tests {
    use crate::decompressor::decompress;
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

        let initial_tree = initial_tree();
        let huffman_tree = compressor::tree::build(initial_tree);

        assert_eq!(
            decompress(
                &compressed_message,
                &huffman_tree.expect("No empty tree possible.")
            ),
            expected_message
        );
    }
}
