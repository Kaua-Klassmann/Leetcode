pub fn is_palindrome(x: i32) -> bool {
    let text = x.to_string();

    text.chars().eq(text.chars().rev())
}

#[cfg(test)]
mod test {
    use crate::is_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(10), false);
    }
}
