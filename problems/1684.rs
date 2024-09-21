pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    words.into_iter().filter(|word: &String| {
        word.chars().all(|c| allowed.contains(c))
    }).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::count_consistent_strings;

    #[test]
    fn test_1() {
        assert_eq!(count_consistent_strings("ab".to_string(),
        vec!["ad","bd","aaab","baa","badab"].iter().map(|s| s.to_string()).collect()),
        2)
    }

    #[test]
    fn test_2() {
        assert_eq!(count_consistent_strings("abc".to_string(),
        vec!["a","b","c","ab","ac","bc","abc"].iter().map(|s| s.to_string()).collect()),
        7)
    }

    #[test]
    fn test_3() {
        assert_eq!(count_consistent_strings("cad".to_string(),
        vec!["cc","acd","b","ba","bac","bad","ac","d"].iter().map(|s| s.to_string()).collect()),
        4)
    }
}
