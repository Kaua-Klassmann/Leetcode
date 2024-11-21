pub fn is_subsequence(s: String, t: String) -> bool {
    let mut t_chars = t.chars();

    for s_char in s.chars() {
        loop {
            match t_chars.next() {
                Some(t_char) => {
                    if t_char == s_char {
                        break;
                    }
                },
                None => return false
            }
        }
    };

    true
}

#[cfg(test)]
mod tests {
    use crate::is_subsequence;

    #[test]
    fn test_1() {
        assert_eq!(is_subsequence("abc".to_string(), "ahbgdc".to_string()), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_subsequence("axc".to_string(), "ahbgdc".to_string()), false)
    }
}