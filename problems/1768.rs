use std::str::Chars;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut resp: Vec<char> = Vec::with_capacity(word1.len() + word2.len());
    let mut chars2: Chars<> = word2.chars();
    let mut chars1: Chars<> = word1.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                resp.push(c1);
                resp.push(c2);
            }
            (Some(c1), None) => {
                resp.push(c1);
                break;
            },
            (None, Some(c2)) => {
                resp.push(c2);
                break;
            },
            (None, None) => break
        }
    }

    resp.extend(chars1);
    resp.extend(chars2);
    
    resp.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::merge_alternately;

    #[test]
    fn test_1() {
        assert_eq!(merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr".to_string())
    }

    #[test]
    fn test_2() {
        assert_eq!(merge_alternately("ab".to_string(), "pqrs".to_string()), "apbqrs".to_string())
    }

    #[test]
    fn test_3() {
        assert_eq!(merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd".to_string())
    }
}
