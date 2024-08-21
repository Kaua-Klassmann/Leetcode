pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums: Vec<i32> = nums;
    nums.sort();
    
    nums.into_iter().step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use crate::array_pair_sum;

    #[test]
    fn test_1() {
        assert_eq!(array_pair_sum(vec![1,4,3,2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(array_pair_sum(vec![6,2,6,5,1,2]), 9);
    }
}