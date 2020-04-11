use std::cmp::Ordering;

#[derive(Eq, Debug)]
pub struct Node {
    pub frequency: u64,
    pub byte: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Copy, Clone)]
pub struct HuffmanCode {
    bin_repres: u8,
    bin_length: u8,
}

impl HuffmanCode {
    pub fn new() -> HuffmanCode {
        HuffmanCode {
            bin_repres: 0,
            bin_length: 0,
        }
    }

    pub fn push_bit(&mut self, set: bool) {
        assert!(self.bin_length < 8, "Attempted to push more than 8 bits.");

        self.bin_repres <<= 1;

        if set {
            self.bin_repres |= 0b00000001;
        }

        self.bin_length += 1;
    }

    pub fn pop_bit(&mut self) -> bool {
        assert!(self.bin_length > 0, "Attempted to extract -1 bit.");

        let popped = (self.bin_repres & 0b00000001) == 1;

        self.bin_repres >>= 1;
        self.bin_length -= 1;

        popped
    }

    pub fn bin_repres(&self) -> u8 {
        self.bin_repres
    }

    pub fn bin_length(&self) -> u8 {
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

#[cfg(test)]
mod tests {
    use crate::huffman::HuffmanCode;

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
}
