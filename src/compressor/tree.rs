use crate::{huffman, stream_helpers};
use std::collections::{BinaryHeap, HashMap};
use std::io::{Read, Seek};

pub fn init<T: Read + Seek>(reader: &mut T) -> std::io::Result<BinaryHeap<Box<huffman::Node>>> {
    let mut byte = [0u8];
    let stream_len = stream_helpers::stream_length(reader)?;

    let mut frequencies = HashMap::<u8, u64>::new();

    while stream_helpers::stream_current_position(reader)? != stream_len {
        reader.read_exact(&mut byte)?;

        match frequencies.get_mut(&byte[0]) {
            Some(frequency) => *frequency += 1,
            None => {
                frequencies.insert(byte[0], 1);
                ()
            }
        }
    }

    let mut result = BinaryHeap::<Box<huffman::Node>>::new();

    for entry in frequencies {
        result.push(Box::new(huffman::Node {
            byte: Some(entry.0),
            frequency: entry.1,
            left: None,
            right: None,
        }));
    }

    Ok(result)
}

pub fn build(mut bin_heap: BinaryHeap<Box<huffman::Node>>) -> Option<Box<huffman::Node>> {
    while bin_heap.len() > 1 {
        let right_node = bin_heap.pop().unwrap();
        let left_node = bin_heap.pop().unwrap();

        let joined_node = huffman::Node {
            frequency: left_node.frequency + right_node.frequency,
            byte: None,
            left: Some(left_node),
            right: Some(right_node),
        };

        bin_heap.push(Box::new(joined_node));
    }

    bin_heap.pop()
}

#[cfg(test)]
mod tests {
    use crate::compressor;
    use crate::huffman::Node;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::fs::File;
    use std::io::BufReader;
    use std::option::Option::Some;
    use std::path::Path;

    fn create_letter_node(letter: char, frequency: u64) -> Box<Node> {
        Box::new(Node {
            frequency,
            byte: Some(letter as u8),
            left: None,
            right: None,
        })
    }

    fn expected_initial_tree() -> BinaryHeap<Box<Node>> {
        let mut bin_heap = BinaryHeap::<Box<Node>>::new();

        bin_heap.push(create_letter_node('A', 2));
        bin_heap.push(create_letter_node('B', 2));
        bin_heap.push(create_letter_node('C', 2));
        bin_heap.push(create_letter_node('D', 3));
        bin_heap.push(create_letter_node('E', 11));
        bin_heap.push(create_letter_node('F', 5));

        bin_heap
    }

    #[test]
    fn test_init() {
        let file_in = File::open(Path::new("test-data/dataset_0/expected.txt"))
            .expect("Could not read test dataset");
        let mut reader = BufReader::new(file_in);

        let initial_tree = compressor::tree::init(&mut reader).expect("Empty tree");

        let mut sorted_by_symbol = initial_tree.into_vec();
        sorted_by_symbol.sort_by(|node_left, node_right| {
            if node_left.byte == node_right.byte {
                Ordering::Equal
            } else if node_left.byte > node_right.byte {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        let expected = expected_initial_tree().into_vec();

        assert_eq!(expected.len(), sorted_by_symbol.len());

        for index in 0..expected.len() {
            assert_eq!(expected.get(index), sorted_by_symbol.get(index));
        }
    }

    fn compare_trees(expected: &Option<Box<Node>>, actual: &Option<Box<Node>>) {
        if let Some(node_expected) = expected {
            let node_actual = actual.as_ref().expect("Actual should not be None");

            assert_eq!(node_expected.byte, node_actual.byte);
            assert_eq!(node_expected.frequency, node_actual.frequency);

            compare_trees(&node_expected.left, &node_actual.left);
            compare_trees(&node_expected.right, &node_actual.right);
        } else if let Some(_node_actual) = actual {
            assert!(false, "Actual node should not be Some");
        }
    }

    fn create_joined_node(frequency: u64, left: Box<Node>, right: Box<Node>) -> Box<Node> {
        Box::new(Node {
            frequency,
            byte: None,
            left: Some(left),
            right: Some(right),
        })
    }

    #[test]
    fn test_build() {
        let initial_tree = expected_initial_tree();
        let huffman_tree = compressor::tree::build(initial_tree);

        let expected_huffman_tree = Box::new(Node {
            frequency: 25,
            byte: None,
            left: Some(Box::new(Node {
                frequency: 14,
                byte: None,
                right: Some(create_joined_node(
                    5,
                    create_letter_node('D', 3),
                    create_letter_node('B', 2),
                )),
                left: Some(Box::new(Node {
                    frequency: 9,
                    byte: None,
                    left: Some(create_letter_node('F', 5)),
                    right: Some(create_joined_node(
                        4,
                        create_letter_node('C', 2),
                        create_letter_node('A', 2),
                    )),
                })),
            })),
            right: Some(create_letter_node('E', 11)),
        });

        compare_trees(&Some(expected_huffman_tree), &huffman_tree);
    }
}
