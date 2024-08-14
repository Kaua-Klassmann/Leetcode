pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = nums.into_iter().map(|num| num.pow(2)).collect();

    output.sort();

    output
}

#[cfg(test)]
mod tests {
    use crate::sorted_squares;

    #[test]
    fn test_1() {
        assert_eq!(sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
    }

    #[test]
    fn test_2() {
        assert_eq!(sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
    }
}