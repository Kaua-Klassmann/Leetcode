use std::str::Chars;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut resp: String = String::new();

    let mut chars: Vec<Chars<'_>> = strs.iter().map(|s| s.chars()).collect();

    loop {
        let letter = match chars[0].next() {
            Some(c) => c,
            None => break
        };

        if chars.iter_mut().skip(1).any(|chs| chs.next() != Some(letter)) {
            break;
        }
        
        resp.push(letter);
    }
    
    resp
}

#[cfg(test)]
mod tests {
    use crate::longest_common_prefix;

    #[test]
    fn test_1() {
        assert_eq!(longest_common_prefix(vec!["flower","flow","flight"].iter().map(|s| s.to_string()).collect()),
        "fl".to_string())
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_common_prefix(vec!["dog","racecar","car"].iter().map(|s| s.to_string()).collect()),
        "".to_string())
    }
}