pub fn count_elements(nums: Vec<i32>) -> i32 {
    let _max: i32 = *nums.iter().max().unwrap();
    let _min: i32 = *nums.iter().min().unwrap();

    nums.into_iter().filter(|&x| x != _max && x != _min).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::count_elements;

    #[test]
    fn test_1() {
        assert_eq!(count_elements(vec![11,7,2,15]), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(count_elements(vec![-3,3,3,90]), 2)
    }
}
