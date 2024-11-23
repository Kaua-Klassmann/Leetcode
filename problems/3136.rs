pub fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }

    let mut vowel = false;
    let mut consonant = false;

    for char in word.chars() {
        match char {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                if vec!['A', 'E', 'I', 'O', 'U'].contains(&char.to_ascii_uppercase()) {
                    vowel = true;
                } else if ('B'..='Z').contains(&char.to_ascii_uppercase()) {
                    consonant = true;
                }
            },
            _ => return false
        }
    }

    vowel && consonant
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("234Adas".to_string()), true)
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid("b3".to_string()), false)
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("a3$e".to_string()), false)
    }
}