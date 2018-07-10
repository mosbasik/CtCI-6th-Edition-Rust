/// sort both strings, then check if they are equal
pub mod approach_1 {
    fn sort_string(s: &str) -> String {
        let mut bytes = s.as_bytes().to_vec();
        bytes.sort_unstable();
        String::from_utf8(bytes).unwrap()
    }

    pub fn permutation(a: &str, b: &str) -> bool {
        sort_string(a) == sort_string(b)
    }
}

/// populate and then tear down a char count structure with the two strings
pub mod approach_2 {
    pub fn permutation(a: &str, b: &str) -> bool {
        // permutations must be the same length
        if a.len() != b.len() {
            return false;
        }

        // assume ASCII strings
        let mut letters: Vec<u8> = vec![0; 128];

        // increment letter counts using first string
        for byte in a.bytes() {
            letters[byte as usize] += 1;
        }

        // decrement letter counts using second string
        for byte in b.bytes() {
            if letters[byte as usize] == 0 {
                return false;
            }
            letters[byte as usize] -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use super::{approach_1, approach_2};

    #[test]
    fn test_1_true() {
        assert_eq!(approach_1::permutation(&"apple", &"papel"), true);
        assert_eq!(approach_1::permutation(&"carrot", &"tarroc"), true);
    }

    #[test]
    fn test_1_false() {
        assert_eq!(approach_1::permutation(&"hello", &"llloh"), false);
    }

    #[test]
    fn test_2_true() {
        assert_eq!(approach_2::permutation(&"apple", &"papel"), true);
        assert_eq!(approach_2::permutation(&"carrot", &"tarroc"), true);
    }

    #[test]
    fn test_2_false() {
        assert_eq!(approach_2::permutation(&"hello", &"llloh"), false);
    }
}
