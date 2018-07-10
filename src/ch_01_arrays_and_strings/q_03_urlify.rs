pub fn urlify(s: String, true_length: usize) -> String {
    // create byte vector by consuming the string in-place
    let mut bytes: Vec<u8> = s.into_bytes();

    let mut space_count: usize = 0;
    for i in 0..true_length {
        if bytes[i] == " ".as_bytes()[0] {
            space_count += 1;
        }
    }

    let mut write_idx: usize = true_length + (space_count * 2);

    for read_idx in (0..true_length).rev() {
        if bytes[read_idx] == " ".as_bytes()[0] {
            bytes[write_idx - 1] = "0".as_bytes()[0];
            bytes[write_idx - 2] = "2".as_bytes()[0];
            bytes[write_idx - 3] = "%".as_bytes()[0];
            write_idx -= 3;
        } else {
            bytes[write_idx - 1] = bytes[read_idx];
            write_idx -= 1;
        }
    }

    // create string from the byte vector in-place
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {

    use super::urlify;

    #[test]
    fn test_example() {
        let s = String::from("Mr John Smith    ");
        assert_eq!(urlify(s, 13), "Mr%20John%20Smith");
    }

    #[test]
    fn test_excess_trailing_spaces() {
        let s = String::from("Mr John Smith          ");
        assert_eq!(urlify(s, 13), "Mr%20John%20Smith      ");
    }
}
