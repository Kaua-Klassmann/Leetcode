pub fn max_number_of_balloons(text: String) -> i32 {
    let mut chars_count: [i32; 5] = [0; 5];

    for c in text.chars() {
        match c {
            'b' => chars_count[0] += 2,
            'a' => chars_count[1] += 2,
            'l' => chars_count[2] += 1,
            'o' => chars_count[3] += 1,
            'n' => chars_count[4] += 2,
            _ => {}
        }
    }    

    *chars_count.iter().min().unwrap() / 2
}

#[cfg(test)]
mod tests {
    use crate::max_number_of_balloons;

    #[test]
    fn test_1() {
        assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1)
    }

    #[test]
    fn test_2() {
        assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2)
    }

    #[test]
    fn test_3() {
        assert_eq!(max_number_of_balloons("leetcode".to_string()), 0)
    }
}