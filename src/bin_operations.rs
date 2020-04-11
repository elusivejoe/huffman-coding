pub fn check_bit_set(stream: &[u8], bit_num: usize) -> Option<bool> {
    if bit_num >= stream.len() * 8 {
        return None;
    }

    let byte_idx = bit_num / 8;
    let relative_bit_idx = bit_num % 8;

    is_bit_set(stream[byte_idx], relative_bit_idx as u8)
}

fn is_bit_set(byte: u8, bit_num: u8) -> Option<bool> {
    if bit_num >= 8 {
        return None;
    }

    let mask = 0b10000000 >> bit_num;

    Some(byte & mask > 0)
}

#[cfg(test)]
mod tests {
    use crate::bin_operations::{check_bit_set, is_bit_set};

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
