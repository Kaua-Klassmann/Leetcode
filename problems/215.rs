pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let tam: usize = nums.len();

    let mut copy = nums.clone();

    *copy.select_nth_unstable(tam - k as usize).1
}

#[cfg(test)]
mod test {
    use crate::find_kth_largest;

    #[test]
    fn test_1() {
        assert_eq!(find_kth_largest(vec![3,2,1,5,6,4], 2), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4)
    }
}