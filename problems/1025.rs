pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::divisor_game;

    #[test]
    fn test_1() {
        assert_eq!(divisor_game(2), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(divisor_game(3), false);
    }
}