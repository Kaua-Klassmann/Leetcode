pub fn length_of_last_word(s: String) -> i32 {
    s.trim().split(" ").last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use crate::length_of_last_word;

    #[test]
    fn test_1() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4)
    }

    #[test]
    fn test_3() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6)
    }
}