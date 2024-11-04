pub fn is_palindrome(s: String) -> bool {
    let (mut l, mut r) = (0 as usize, s.len() - 1);

    while l < r {
        while l < r && !s.as_bytes()[l].is_ascii_alphanumeric() {
            l += 1;
        }

        while l < r && !s.as_bytes()[r].is_ascii_alphanumeric() {
            r -= 1;
        }

        if s.as_bytes()[l].to_ascii_lowercase() == s.as_bytes()[r].to_ascii_lowercase() {
            l += 1;
            r = r.saturating_sub(1);
            continue;
        }
        
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_1() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_palindrome("race a car".to_string()), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(" ".to_string()), true)
    }
}