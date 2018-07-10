// Assumption: input is an ASCII string.  This means A) we can use len() because
// the string's length in bytes and in characters are equal (because the first
// 128 Unicode characters *are* the 128 one-byte ASCII characters) and B) there
// are only 128 possible unique characters,

/// Approach 1: record character occurances in a vector of 128 booleans
pub fn is_unique_chars_1(string: &str) -> bool {
    if string.len() > 128 {
        return false;
    }

    let mut bool_vector = vec![false; 128];

    for byte in string.bytes() {
        if bool_vector[byte as usize] {
            return false;
        }
        bool_vector[byte as usize] = true;
    }
    true
}

/// Approach 2: record character occurances in a 128-bit unsigned integer
pub fn is_unique_chars_2(string: &str) -> bool {
    if string.len() > 128 {
        return false;
    }

    let mut bit_array: u128 = 0;

    for byte in string.bytes() {
        let mask = 1 << byte;
        if bit_array & mask != 0 {
            return false;
        }
        bit_array |= mask;
    }
    true
}

#[cfg(test)]
mod tests {

    use super::{is_unique_chars_1, is_unique_chars_2};

    #[test]
    fn test_1_true() {
        assert_eq!(is_unique_chars_1(&"abcd"), true);
        assert_eq!(is_unique_chars_1(&"s4fad"), true);
    }

    #[test]
    fn test_1_false() {
        assert_eq!(is_unique_chars_1(&"23ds2"), false);
        assert_eq!(is_unique_chars_1(&"hb 627jh=j ()"), false);
    }

    #[test]
    fn test_2_true() {
        assert_eq!(is_unique_chars_2(&"abcd"), true);
        assert_eq!(is_unique_chars_2(&"s4fad"), true);
    }

    #[test]
    fn test_2_false() {
        assert_eq!(is_unique_chars_2(&"23ds2"), false);
        assert_eq!(is_unique_chars_2(&"hb 627jh=j ()"), false);
    }
}
