pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(i) => i as i32,
        None => -1
    }
}

#[cfg(test)]
mod tests {
    use crate::str_str;

    #[test]
    fn test_1() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0)
    }

    #[test]
    fn test_2() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1)
    }
}