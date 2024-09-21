use std::collections::HashSet;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let set: HashSet<char> = jewels.chars().collect();

    stones.chars().filter(|stone| set.contains(stone)).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::num_jewels_in_stones;

    #[test]
    fn test_1() {
        assert_eq!(num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0)
    }
}
