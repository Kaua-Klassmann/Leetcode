use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    const MAP: [&str; 26] = [
        ".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--",
        "-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."
    ];

    let mut set: HashSet<String> = HashSet::new();

    for word in words {
        let code: String = word.chars().into_iter()
                .map(|c| MAP[c as usize - 97]).collect();

        set.insert(code);
    }

    set.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::unique_morse_representations;

    #[test]
    fn test_1() {
        assert_eq!(unique_morse_representations(vec!["gin","zen","gig","msg"].iter().map(|s| s.to_string()).collect()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(unique_morse_representations(vec!["a".to_string()]), 1)
    }
}