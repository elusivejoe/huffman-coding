use crate::{huffman, utils};
use std::collections::{BinaryHeap, HashMap};
use std::io::{Read, Seek};

pub fn init<T: Read + Seek>(reader: &mut T) -> std::io::Result<BinaryHeap<Box<huffman::Node>>> {
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
    #[test]
    fn test_init() {
        unimplemented!("!");
    }

    #[test]
    fn test_build() {
        unimplemented!("!");
    }
}
