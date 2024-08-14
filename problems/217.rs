use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    nums.into_iter().any(|num| !set.insert(num))
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate;

    #[test]
    fn test_1() {
        assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(contains_duplicate(vec![1,2,3,4]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
}