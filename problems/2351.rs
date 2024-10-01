pub fn repeated_character(s: String) -> char {
    let mut set: u32 = 0;

    for c in s.chars() {
        let bit_masket: u32 = 1 << (c as u32 - 97);

        if set & bit_masket != 0 {
            return c;
        }

        set |= bit_masket;
    }

    ' '
}

#[cfg(test)]
mod test {
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
