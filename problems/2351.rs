use std::collections::HashSet;

pub fn repeated_character(s: String) -> char {
    let mut set: HashSet<char> = HashSet::with_capacity(26);

    for c in s.chars() {
        if set.contains(&c) {
            return c;
        }

        set.insert(c);
    }

    ' '
}

#[cfg(test)]
mod tests {
    use crate::repeated_character;

    #[test]
    fn test_1() {
        assert_eq!(repeated_character("abccbaacz".to_string()), 'c')
    }

    #[test]
    fn test_2() {
        assert_eq!(repeated_character("abcdd".to_string()), 'd')
    }
}