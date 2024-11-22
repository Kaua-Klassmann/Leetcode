pub fn number_of_special_chars(word: String) -> i32 {
    let mut map: [bool; 128] = [false; 128];

    for &byte in word.as_bytes() {
        map[byte as usize] = true;
    }

    (b'A'..=b'Z').filter(|&byte| map[byte as usize] && map[(byte as usize + 32) as usize]).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::number_of_special_chars;

    #[test]
    fn test_1() {
        assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(number_of_special_chars("abc".to_string()), 0)
    }

    #[test]
    fn test_3() {
        assert_eq!(number_of_special_chars("abBcab".to_string()), 1)
    }
}