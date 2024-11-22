pub fn score_of_string(s: String) -> i32 {
    let mut sum: i32 = 0;

    for i in 1..s.len() as usize {
        sum += s.as_bytes()[i-1].abs_diff(s.as_bytes()[i]) as i32
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::score_of_string;

    #[test]
    fn test_1() {
        assert_eq!(score_of_string("hello".to_string()), 13)
    }

    #[test]
    fn test_2() {
        assert_eq!(score_of_string("zaz".to_string()), 50)
    }
}