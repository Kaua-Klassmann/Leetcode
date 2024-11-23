pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().filter(|&num| num < k).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::min_operations;

    #[test]
    fn test_1() {
        assert_eq!(min_operations(vec![2,11,10,1,3], 10), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(min_operations(vec![1,1,2,4,9], 1), 0)
    }

    #[test]
    fn test_3() {
        assert_eq!(min_operations(vec![1,1,2,4,9], 9), 4)
    }
}